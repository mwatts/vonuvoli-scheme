

extern crate rust_scheme;

use rust_scheme::exports::*;




#[ test ]
fn test () -> () {
	
	let tests = vec! [
			
			/*== booleans ==*/
			"#t", "#true", "#f", "#false",
			
			/*== numbers ==*/
			"0", "+0", "-0",
			"01", "+01", "-01",
			"1234567890", "+1234567890", "-1234567890",
			"#b0", "#b01", "#b10", "#B0", "#B01", "#B10",
			"#o0", "#o07", "#o70", "#O0", "#O07", "#O70",
			"#d0", "#d01", "#d10", "#D0", "#D01", "#D10",
			"#x0", "#x0f", "#xf0", "#X0", "#X0f", "#Xf0",
			"#b-0", "#b-01", "#b-10", "#B-0", "#B-01", "#B-10",
			"#o-0", "#o-07", "#o-70", "#O-0", "#O-07", "#O-70",
			"#d-0", "#d-01", "#d-10", "#D-0", "#D-01", "#D-10",
			"#x-0", "#x-0f", "#x-f0", "#X-0", "#X-0f", "#X-f0",
			"#b+0", "#b+01", "#b+10", "#B+0", "#B+01", "#B+10",
			"#o+0", "#o+07", "#o+70", "#O+0", "#O+07", "#O+70",
			"#d+0", "#d+01", "#d+10", "#D+0", "#D+01", "#D+10",
			"#x+0", "#x+0f", "#x+f0", "#X+0", "#X+0f", "#X+f0",
			"0.0", "0.", ".0", "+0.0", "+0.", "+.0", "-0.0", "-0.", "-.0",
			"1.2", "1.", ".2", "+1.2", "+1.", "+.2", "-1.2", "-1.", "-.2",
			"0.0e0", "0.e+0", ".0e-0", "+0.0e0", "+0.e+0", "+.0e-0", "-0.0e0", "-0.e+0", "-.0e-0",
			"1.2e3", "1.e+3", ".2e-3", "+1.2e3", "+1.e+3", "+.2e-3", "-1.2e3", "-1.e+3", "-.2e-3",
			"+inf", "+inf.0", "-inf", "-inf.0",
			"+nan", "+nan.0", "-nan", "-nan.0",
			"+epsilon", "+epsilon.0", "-epsilon", "-epsilon.0",
			
			/*== symbols 1 ==*/
			"a", "A", "ab", "aB", "Ab", "AB",
			"a1", "1a",
			"+", "-", "+a", "-a", ".a", "a+", "a-", "a.",
			
			/*== symbols 2 ==*/
			"|#t|", "|#true|", "|#f|", "|#false|",
			"|0|", "|+0|", "|-0|",
			"|()|", "|#null|",
			r#"||"#, r#"|a|"#, r#"|0|"#, r#"|a0|"#, r#"|0a|"#, r#"| |"#, r#"|a a|"#, r#"|\n|"#, r#"|\x0;|"#, r#"|\x00;|"#, r#"|\x000;|"#,
			
			/*== characters ==*/
			"#\\x", "#\\x0", "#\\x00", "#\\x000", "#\\null",
			
			/*== strings ==*/
			r#""""#, r#""a""#, r#""0""#, r#""a0""#, r#""0a""#, r#"" ""#, r#""a a""#, r#""\n""#, r#""\x0;""#, r#""\x00;""#, r#""\x000;""#,
			"#string()", "#string( )", "#string(0)", "#string( 0 )", "#string(0 1)", "#string(0 1 2)", "#string(0 1 2 3)",
			
			/*== bytes ==*/
			"#u8()", "#u8( )", "#u8(0)", "#u8( 0 )", "#u8(0 1)", "#u8(0 1 2)", "#u8(0 1 2 3)",
			"#U8()", "#U8( )", "#U8(0)", "#U8( 0 )", "#U8(0 1)", "#U8(0 1 2)", "#U8(0 1 2 3)",
			"#bytes()", "#bytes( )", "#bytes(0)", "#bytes( 0 )", "#bytes(0 1)", "#bytes(0 1 2)", "#bytes(0 1 2 3)",
			
			/*== lists ==*/
			"()", "#null",
			"(0)", "( 0 )", "(0 1)", "(0 1 2)",
			"(0 . 1)", "( 0 . 1 )",
			"(0 1 . 2)", "(0 1 2 . 3)",
			
			/*== vectors ==*/
			"#()", "#( )", "#(0)", "#( 0 )", "#(0 1)", "#(0 1 2)", "#(0 1 2 3)",
			"#array()", "#array( )", "#array(0)", "#array( 0 )", "#array(0 1)", "#array(0 1 2)", "#array(0 1 2 3)",
			
			/*== values ==*/
			"#values()", "#values( )", "#values(0)", "#values( 0 )", "#values(0 1)", "#values(0 1 2)", "#values(0 1 2 3)",
			
		];
	
	
	for input_1 in tests {
		
		println! (">> `{}`", input_1);
		
		let output_1 = parse_value (input_1) .expect ("9f292a07");
		let input_2 = output_1.to_string ();
		let output_2 = parse_value (input_2.as_ref ()) .expect ("3652725f");
		
		println! ("== `{}` -> `{}` -> `{}`", input_1, output_1, output_2);
		println! ("## {:#?}", output_1);
		println! ();
	}
	
}

