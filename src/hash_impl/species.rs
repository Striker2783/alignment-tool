use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Species {
    pub(crate) name: String,
    pub(crate) kingdom: String,
    pub(crate) phylum: String,
    pub(crate) class: String,
    pub(crate) order: String,
    pub(crate) family: String,
    pub(crate) genus: String,
    pub(crate) species: String,
    pub(crate) genome: String,
}

impl Species {
    fn set_part(&mut self, string: &str) -> Result<(), Box<dyn Error>> {
        let Some(first) = string
            .chars()
            .next() else {
            return Err(format!("Failed to get first character for {string}"))?};

        let rest = string[3..].to_owned();
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
        thing.name = name.to_owned();

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
        assert_eq!(default.kingdom, "Eukaryota");
        let _ = default.set_part("p__Chordata");
        assert_eq!(default.phylum, "Chordata");
        let _ = default.set_part("c__Mammalia");
        assert_eq!(default.class, "Mammalia");
        let _ = default.set_part("o__Dasyuromorphia");
        assert_eq!(default.order, "Dasyuromorphia");
        let _ = default.set_part("g__Murexia");
        assert_eq!(default.genus, "Murexia");
        let _ = default.set_part("s__Murexia longicaudata");
        assert_eq!(default.species, "Murexia longicaudata");
    }
    #[test]
    fn build() {
        let content = "KF294262.1	k__Eukaryota;p__Chordata;c__Mammalia;o__Dasyuromorphia;f__Dasyuridae;g__Murexia;s__Murexia longicaudata;";
        let test = Species::build(content).expect("");
        let expected = Species {
            name: "KF294262.1".to_owned(),
            kingdom: "Eukaryota".to_owned(),
            phylum: "Chordata".to_owned(),
            class: "Mammalia".to_owned(),
            order: "Dasyuromorphia".to_owned(),
            family: "Dasyuridae".to_owned(),
            genus: "Murexia".to_owned(),
            species: "Murexia longicaudata".to_owned(),
            genome: "".to_owned(),
        };
        assert_eq!(test, expected);
    }
}
