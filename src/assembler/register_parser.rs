use crate::nom::types::CompleteStr;

use crate::nom::digit;
use crate::assembler::Token;

named!(register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >> reg_num: digit
            >> (Token::Register{reg_num: reg_num.parse::<u8>().unwrap()})
        )
    )
);


mod tests {
    use super::*;
    use crate::nom::types::CompleteStr;
    use crate::assembler::Token;
    use crate::instructions::Opcode;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }

}