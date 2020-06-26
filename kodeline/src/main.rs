use anyhow::Result;

mod client;
mod constants;
mod deser;
mod octachan;

use crate::octachan::Octachan;
use std::collections::HashMap;

#[macro_use]
extern crate prettytable;

use prettytable::{Cell, Row, Table};

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::get_authenticated_client()?;
    //   let user_basic = format!("{}{}", constants::BASE_URL, "/user");
    //   let user = Octachan::get_as::<deser::User>(&user_basic, &client).await?;
    //
    //   let user_basic = format!("{}{}", constants::BASE_URL, "/users/ferbncode");
    //  println!("User is {:?}", user);
    // let user = Octachan::get_as::<deser::User>(&user_basic, &client).await?;

    //    println!("User is {:?}", user);
    //
    //
    //  MAKE THIS RUN WITH ASYNCIO
    //
    //
    let repos_url = format!("{}{}", constants::BASE_URL, "/users/clearnote01/repos");
    let repos = Octachan::get_as::<Vec<deser::Repo>>(&repos_url, &client).await?;
    let mut table = Table::new();
    table.add_row(row![bFg->"Repository", Fyb->"Fork", Fbb->"Languages"]);
    for repo in repos {
        let langs: HashMap<String, i32> =
            Octachan::get_as::<deser::LanguageStats>(&repo.languages_url, &client).await?;
        let mut lang_table = Table::new();
        let mut lang_row = Vec::<Cell>::new();
        let mut size_row = Vec::<Cell>::new();
        for (key, &val) in langs.iter() {
            lang_row.push(Cell::new(key).style_spec("bFg"));
            size_row.push(Cell::new(&val.to_string()).style_spec("bFg"));
        }
        lang_table.add_row(Row::new(lang_row));
        lang_table.add_row(Row::new(size_row));

        table.add_row(row!(Fgb->repo.full_name, c->repo.fork, lang_table));
    }
    table.printstd();

    Ok(())
}
