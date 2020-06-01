use barrel::{types, Migration};
use barrel::backend::Pg;

pub fn migration() -> String {
    let mut m = Migration::new();
    println!("Applying: {}", file!());

    m.create_table("todos", |t| {
        t.add_column("id", types::primary());
        t.add_column("title", types::varchar(255));
        t.add_column("is_completed", types::boolean().default(false));
    });

    m.make::<Pg>()
}
