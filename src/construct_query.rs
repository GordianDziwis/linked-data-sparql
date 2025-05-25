use crate::and::And;
use crate::join::Join;
use crate::to_construct_query::ToConstructQuery;
use crate::union::Union;
use spargebra::Query;
use spargebra::algebra::{Expression, GraphPattern};
use spargebra::term::{NamedNode, NamedNodePattern, TermPattern, TriplePattern, Variable};
use sparopt::Optimizer;

#[derive(Default)]
pub struct ConstructQuery {
  construct_template: Vec<TriplePattern>,
  where_pattern: GraphPattern,
}

impl ConstructQuery {
  pub fn new(
    subject: impl Into<TermPattern>,
    predicate: impl Into<NamedNodePattern>,
    object: impl Into<TermPattern>,
  ) -> Self {
    let patterns = vec![TriplePattern {
      subject: subject.into(),
      predicate: predicate.into(),
      object: object.into(),
    }];

    Self {
      construct_template: patterns.clone(),
      where_pattern: GraphPattern::Bgp { patterns },
    }
  }

  pub fn new_with_binding<F>(
    subject: Variable,
    predicate: NamedNode,
    to_query_with_binding: F,
  ) -> Self
  where
    F: FnOnce(Variable) -> Self,
  {
    let object = Variable::new_unchecked(spargebra::term::BlankNode::default().into_string());

    Self::new(subject, predicate, object.clone()).join(to_query_with_binding(object))
  }

  pub fn union_with_binding<F>(
    self,
    subject: Variable,
    predicate: NamedNode,
    to_query_with_binding: F,
  ) -> Self
  where
    F: FnOnce(Variable) -> Self,
  {
    let object = Variable::new_unchecked(spargebra::term::BlankNode::default().into_string());

    self
      .union(Self::new(subject, predicate, object.clone()))
      .join(to_query_with_binding(object))
  }

  pub fn join_with_binding<F>(
    self,
    subject: Variable,
    predicate: NamedNode,
    to_query_with_binding: F,
  ) -> Self
  where
    F: FnOnce(Variable) -> Self,
  {
    let object = Variable::new_unchecked(spargebra::term::BlankNode::default().into_string());

    self
      .join(Self::new(subject, predicate, object.clone()))
      .join(to_query_with_binding(object))
  }

  pub fn join_with(self, subject: Variable, predicate: NamedNode, object: NamedNode) -> Self {
    self.join(Self::new(subject, predicate, object))
  }

  pub fn filter_variable(self, variable: Variable, id: NamedNode) -> Self {
    let expr = Expression::Equal(
      Box::new(Expression::Variable(variable)),
      Box::new(Expression::NamedNode(id)),
    );
    Self {
      construct_template: self.construct_template,
      where_pattern: GraphPattern::Filter {
        expr,
        inner: Box::new(self.where_pattern),
      },
    }
  }
}

impl From<ConstructQuery> for Query {
  fn from(value: ConstructQuery) -> Self {
    let pattern = (&Optimizer::optimize_graph_pattern((&value.where_pattern).into())).into();

    Query::Construct {
      template: value.construct_template,
      dataset: None,
      pattern,
      base_iri: None,
    }
  }
}

impl ToConstructQuery for Variable {
  fn to_query_with_binding(_: Variable) -> ConstructQuery {
    ConstructQuery::default()
  }
}

macro_rules! to_construct_query_datatypes {
    ($($t:ty),*) => {
        $(
            impl ToConstructQuery for $t {
                fn to_query_with_binding(_: Variable) -> ConstructQuery {
                    ConstructQuery::default()
                }
            }
        )*
    };
}

to_construct_query_datatypes!(
  u8,
  u16,
  u32,
  u64,
  i8,
  i16,
  i32,
  i64,
  String,
  xsd_types::DateTime
);

impl Join for ConstructQuery {
  fn join(mut self, other: Self) -> Self {
    self.construct_template.extend(other.construct_template);
    self.where_pattern = self.where_pattern.join(other.where_pattern);
    self
  }
}

impl Union for ConstructQuery {
  fn union(mut self, other: Self) -> Self {
    self.construct_template.extend(other.construct_template);
    self.where_pattern = self.where_pattern.union(other.where_pattern);
    self
  }
}

impl And for Vec<TriplePattern> {
  fn and(mut self, other: Self) -> Self {
    self.extend(other);
    self
  }
}

impl Join for GraphPattern {
  fn join(self, other: Self) -> Self {
    GraphPattern::Join {
      left: Box::new(self),
      right: Box::new(other),
    }
  }
}

impl Union for GraphPattern {
  fn union(self, other: Self) -> Self {
    GraphPattern::Union {
      left: Box::new(self),
      right: Box::new(other),
    }
  }
}
