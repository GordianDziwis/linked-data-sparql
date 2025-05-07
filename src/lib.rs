mod and;
mod construct_query;
mod join;
mod sparql_query;
mod to_construct_query;
mod union;

pub use crate::and::And;
pub use crate::construct_query::ConstructQuery;
pub use crate::join::Join;
pub use crate::sparql_query::SparqlQuery;
pub use crate::to_construct_query::ToConstructQuery;
pub use crate::union::Union;
pub use linked_data_sparql_derive::Sparql;
use spargebra::Query;
use spargebra::term::{NamedNode, Variable};

pub mod reexport {
  pub use spargebra;
}

impl<T> SparqlQuery for T
where
  T: ToConstructQuery,
{
  fn sparql_algebra() -> Query {
    Self::to_query().into()
  }
}

pub fn with_predicate<F>(
  predicate: NamedNode,
  to_query_with_binding: F,
) -> impl FnOnce(Variable) -> ConstructQuery
where
  F: FnOnce(Variable) -> ConstructQuery,
{
  |subject| {
    let object = Variable::new_unchecked(spargebra::term::BlankNode::default().into_string());

    ConstructQuery::new(subject, predicate, object.clone()).join(to_query_with_binding(object))
  }
}
