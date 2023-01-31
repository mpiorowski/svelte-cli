use ::anyhow::Result;

pub enum Operation {
    Print(Option<String>),
    Page(String, String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self> {
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).unwrap().to_string();
        if term == "add" {
            if value.len() < 3 {
                return Err(anyhow::anyhow!("page requires a name and a path"));
            }
            let mut drain = value.drain(1..=2);

            Ok(Operation::Page(
                drain.next().expect("page name"),
                drain.next().expect("page path"),
            ))
        } else {
            Ok(Operation::Print(Some(term)))
        }
    }
}

fn get_config() -> Result<Config> {
    let config = Config::try_from(Opts::parse().args)?;
    Ok(config)
}
