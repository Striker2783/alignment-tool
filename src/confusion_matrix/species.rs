use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Species {
    pub(crate) name: String,
    pub(crate) kingdom: Option<String>,
    pub(crate) phylum: Option<String>,
    pub(crate) class: Option<String>,
    pub(crate) order: Option<String>,
    pub(crate) family: Option<String>,
    pub(crate) genus: Option<String>,
    pub(crate) species: Option<String>,
}

impl Species {
    #[inline]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
    fn set_part(&mut self, string: &str) -> Result<(), Box<dyn Error>> {
        let Some(first) = string
            .chars()
            .next() else {
            return Err(format!("Failed to get first character for {string}"))?};

        let rest = Some(string[3..].to_owned());
        match first {
            'k' => self.kingdom = rest,
            'p' => self.phylum = rest,
            'c' => self.class = rest,
            'o' => self.order = rest,
            'f' => self.family = rest,
            'g' => self.genus = rest,
            's' => self.species = rest,
            _ => (),
        };
        Ok(())
    }
    pub fn build(line: &str) -> Result<Self, Box<dyn Error>> {
        let mut thing = Self::default();

        let space = line.find('\t').ok_or("A")?;
        let name = &line[..space];
        thing.set_name(name);

        let details = &line[(space + 1)..(line.len() - 1)];
        let split = details.split(';');

        for x in split {
            let _ = thing.set_part(x);
        }

        Ok(thing)
    }
}
#[cfg(test)]
mod test {

    use super::Species;

    #[test]
    fn set_part() {
        let mut default = Species::default();
        let _ = default.set_part("k__Eukaryota");
        assert_eq!(default.kingdom, Some("Eukaryota".to_string()));
        let _ = default.set_part("p__Chordata");
        assert_eq!(default.phylum, Some("Chordata".to_string()));
        let _ = default.set_part("c__Mammalia");
        assert_eq!(default.class, Some("Mammalia".to_string()));
        let _ = default.set_part("o__Dasyuromorphia");
        assert_eq!(default.order, Some("Dasyuromorphia".to_string()));
        let _ = default.set_part("g__Murexia");
        assert_eq!(default.genus, Some("Murexia".to_string()));
        let _ = default.set_part("s__Murexia longicaudata");
        assert_eq!(default.species, Some("Murexia longicaudata".to_string()));
    }
    #[test]
    fn build() {
        let content = "KF294262.1	k__Eukaryota;p__Chordata;c__Mammalia;o__Dasyuromorphia;f__Dasyuridae;g__Murexia;s__Murexia longicaudata;";
        let test = Species::build(content).expect("");
        let expected = Species {
            name: "KF294262.1".to_owned(),
            kingdom: Some("Eukaryota".to_owned()),
            phylum: Some("Chordata".to_owned()),
            class: Some("Mammalia".to_owned()),
            order: Some("Dasyuromorphia".to_owned()),
            family: Some("Dasyuridae".to_owned()),
            genus: Some("Murexia".to_owned()),
            species: Some("Murexia longicaudata".to_owned()),
        };
        assert_eq!(test, expected);
    }
}
