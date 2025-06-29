use crate::token::TokenType;
use crate::tokenizer::Tokenizer;

use crate::expression::Expression;
use crate::until;



macro_rules! comp {
    [tuple_item_1:tt, tuple_item_2:tt; for x in expr] => {
        {
            let mut res = Vec::new();
            for x in expr {
                res.push((tuple_item_1, tuple_item_2));
            }
            res
        }
    };
    [$value:expr; until $cond:expr] => {
        {
            let mut res = Vec::new();
            while !$cond {
                res.push($value);
            }
            res
        }
    };

}

pub struct Type_ {
    pub name: String,
    pub sub_types: Vec<Type_>,
}

impl Type_ {
    pub fn new(t: &mut Tokenizer) -> Self {
        
        let mut res = Self { name: t.expect(TokenType::IDENTIFIER).to_string(), 
         sub_types: vec![]};
         if t.optionaly_expect_char('<') {
             until!(t.optionaly_expect_char('>'); {
                 res.sub_types.push(Type_::new(t));
                 t.optionaly_expect_char(',');
             });
             
         }
        res
    }
    pub fn display(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        match self.sub_types.len() {
            0 => return self.name.clone(),
            1 => return format!("{}<type: {}>", self.name, self.sub_types[0].to_string()),
            2 => return format!("{}<type: {}, type: {}>", self.name, self.sub_types[0].to_string(), self.sub_types[1].to_string()),
            _ => return format!("{}<type: {}, type: {}, type: {}>", self.name, self.sub_types[0].to_string(), self.sub_types[1].to_string(), self.sub_types[2].to_string())
            
        }
        
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut t = Tokenizer {
            code: "    
            Person<int, string<char>>
            "
            .to_string(),
            parse_index: 0,
        };



        let _type = Type_::new(&mut t);
        _type.display();

        assert_eq!(_type.name, "Person");
        assert_eq!(_type.sub_types.len(), 2);
        assert_eq!(_type.sub_types[0].name, "int");
        
    }
}