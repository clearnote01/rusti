use anyhow::Result;
mod client;
mod constants;
mod deser;
mod octachan;

use crate::octachan::Octachan;
use futures::future::join_all;

#[macro_use]
extern crate clap;

use deser::*;

#[macro_use]
extern crate prettytable;

use prettytable::{Cell, Row, Table};

struct RepoStat {
    full_name: String,
    fork: bool,
    langs: LanguageStats,
}

async fn fetch_repo_stat(repo: Repo, client: &reqwest::Client) -> Result<RepoStat> {
    let langs: LanguageStats =
        Octachan::get_as::<deser::LanguageStats>(&repo.languages_url, &client).await?;
    Ok(RepoStat {
        langs: langs,
        full_name: repo.full_name,
        fork: repo.fork,
    })
}

fn render_table_repo_stats(repo_stat_result: Vec<Result<RepoStat>>) -> Result<()> {
    let mut table = Table::new();
    table.add_row(row![bFg->"Repository", Fbb->"Fork", Fyb->"Languages"]);
    for repo_stat in repo_stat_result {
        let repo_stat = repo_stat?;
        let mut lang_row = Vec::<Cell>::new();
        let mut size_row = Vec::<Cell>::new();
        for (key, &val) in repo_stat.langs.iter() {
            lang_row.push(Cell::new(key).style_spec("bFg"));
            size_row.push(Cell::new(&val.to_string()).style_spec("bFg"));
        }
        let mut lang_table = Table::new();
        lang_table.add_row(Row::new(lang_row));
        lang_table.add_row(Row::new(size_row));

        let fork_cell = match repo_stat.fork {
            true => Cell::new("Fork").style_spec("bFg"),
            false => Cell::new("-").style_spec("FrbFo"),
        };

        table.add_row(row!(Fgb->repo_stat.full_name, Fb->fork_cell, Fyb->lang_table));
    }
    table.printstd();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::get_authenticated_client()?;

    let matches = clap_app!(myapp =>
        (version: "1.0.0")
        (author: "clearnote01")
        (about: "See github stats")
        (@arg USER_NAME: +required "Give github username")
    )
    .get_matches();
    let user_name = matches.value_of("USER_NAME").expect("Give github username");

    let route = format!("/users/{}/repos", user_name);
    let repos_url = format!("{}{}", constants::BASE_URL, route);
    let repos = Octachan::get_as::<Vec<deser::Repo>>(&repos_url, &client).await?;
    let repos_count = repos.len();

    let mut futures = vec![];

    for repo in repos {
        // memory for repo is moved here to the future
        futures.push(fetch_repo_stat(repo, &client));
    }

    let repo_stat_result = join_all(futures).await;
    let _ = render_table_repo_stats(repo_stat_result);
    println!("Total repositories found : {}", repos_count);
    Ok(())
}
