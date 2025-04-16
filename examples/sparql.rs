use linked_data_sparql::Sparql;

#[derive(Sparql, Debug)]
#[ld(prefix("ex" = "http://ex/"))]
enum Enum {
    #[ld("ex:left")]
    Left(String),

    #[ld("ex:right")]
    Right(String),
}

fn main() {
    // println!("{}", Book::get_sparql());
    // println!("{}", Foo::get_sparql());
}
