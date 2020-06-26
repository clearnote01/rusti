use url::Url;
use anyhow::Result;
use anyhow::format_err;

pub struct Octachan {}

impl Octachan {
    pub async fn get_as<T: std::fmt::Debug + serde::de::DeserializeOwned>(
        route: &str,
        client: &reqwest::Client,
    ) -> Result<T> {
        println!("Calling route at : {}", route);
        let formed_url = Url::parse(route)?;
        let result: T = Octachan::get_and_deser::<T>(&client, &formed_url).await?;
        Ok(result)
    }

    async fn get_and_deser<'a, T: std::fmt::Debug + serde::de::DeserializeOwned>(
        client: &reqwest::Client,
        url: &Url,
    ) -> Result<T> {
//        let text = client.get(url).send().await?;
        //let text = text.json::<T>().await?;
        let resp = client.get(url.as_str()).send().await?;
        match resp.json::<T>().await {
            Ok(json) => {
                Ok(json)
            },
            Err(msg) => {
                println!("Text: {:?}", client.get(url.as_str()).send().await?.text().await?);
                Err(format_err!(msg))
            }
        }
        //Ok(text)
    }
        ////////////////////////////////////////////////////////////////////////////
        // match text {                                                           //
        //     Ok(text) => text,                                                  //
        //     Err(msg) => {                                                      //
        //         println!("Failed with message : {:?} for input \n {:?}", msg); //
        //         let text = client.get(url).send().await?.text().await?;        //
        //         return Err(msg);                                               //
        //     }                                                                  //
        // }                                                                      //
        ////////////////////////////////////////////////////////////////////////////
}
