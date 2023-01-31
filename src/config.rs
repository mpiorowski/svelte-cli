use ::anyhow::Result;

pub enum Operation {
    Print(Option<String>),
    Page(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        if value.len() == 0 {
            return Ok(Operation::Print(None))
        }

        let term = value.get(0).unwrap().to_string();
        if term == "add" {
            if value.len() < 2 {
                return Err(anyhow::anyhow!("page requires a page name"))
            }
            let page = value.get(1).unwrap().to_string();
            Ok(Operation::Page(page))
        } else {
            Ok(Operation::Print(Some(term)))
        }
    }
}
