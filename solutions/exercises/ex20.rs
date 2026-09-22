#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub name: String,
}
impl Record {
    pub fn parse(line: &str) -> Result<Self, String> {
        if line.contains(['\r', '\n']) {
            return Err("line break".into());
        }
        let (id, name) = line.split_once('\t').ok_or("missing tab")?;
        if name.trim().is_empty() || name.contains('\t') {
            return Err("invalid name".into());
        }
        Ok(Self {
            id: id.parse::<u32>().map_err(|e| e.to_string())?,
            name: name.into(),
        })
    }
    pub fn format(&self) -> String {
        format!("{}\t{}", self.id, self.name)
    }
}
