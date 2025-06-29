use diesel::prelude::*;
use serde::Serialize;

use crate::schema::authors;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub birth_year: i32,
    pub country: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String,
    pub birth_year: i32,
    pub country: String,
}

impl NewAuthor {
    pub fn new(name: &str, birth_year: i32, country: &str) -> Self {
        NewAuthor {
            name: name.to_string(),
            birth_year,
            country: country.to_string(),
        }
    }
}

impl Author {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_birth_year(&self) -> i32 {
        self.birth_year
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Author: {} (ID: {})", self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_author_creation() {
        let new_author = NewAuthor::new("Machado de Assis", 1839, "Brazil");
        assert_eq!(new_author.name, "Machado de Assis");
        assert_eq!(new_author.birth_year, 1839);
        assert_eq!(new_author.country, "Brazil");
    }

    #[test]
    fn test_author_display() {
        let new_author = NewAuthor::new("Machado de Assis", 1839, "Brazil");
        let author = Author {
            id: 1,
            name: new_author.name,
            birth_year: new_author.birth_year,
            country: new_author.country,
        };
        let display_str = format!("{}", author);
        assert_eq!(display_str, "Author: Machado de Assis (ID: 1)");
    }
}
