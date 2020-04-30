SOURCE_FILE@[0; 128)
  IMPORT_DEF@[0; 26)
    IMPORT_KW@[0; 6) "import"
    WHITESPACE@[6; 7) " "
    IMPORT_SEGMENT@[7; 10)
      NAME@[7; 10)
        IDENT@[7; 10) "foo"
    COLON_COLON@[10; 12) "::"
    IMPORT_SEGMENT@[12; 15)
      NAME@[12; 15)
        IDENT@[12; 15) "bar"
    COLON_COLON@[15; 17) "::"
    IMPORT_LIST@[17; 22)
      L_CURLY@[17; 18) "{"
      IMPORT_SEGMENT@[18; 21)
        NAME@[18; 21)
          IDENT@[18; 21) "bar"
      R_CURLY@[21; 22) "}"
    SEMI@[22; 23) ";"
    WHITESPACE@[23; 26) "\n\n\n"
  MOD_DEF@[26; 35)
    MOD_KW@[26; 29) "mod"
    WHITESPACE@[29; 30) " "
    NAME@[30; 33)
      IDENT@[30; 33) "bar"
    SEMI@[33; 34) ";"
    WHITESPACE@[34; 35) "\n"
  MOD_DEF@[35; 44)
    MOD_KW@[35; 38) "mod"
    WHITESPACE@[38; 39) " "
    NAME@[39; 42)
      IDENT@[39; 42) "foo"
    SEMI@[42; 43) ";"
    WHITESPACE@[43; 44) "\n"
  MOD_DEF@[44; 53)
    MOD_KW@[44; 47) "mod"
    WHITESPACE@[47; 48) " "
    NAME@[48; 51)
      IDENT@[48; 51) "baz"
    SEMI@[51; 52) ";"
    WHITESPACE@[52; 53) "\n"
  MOD_DEF@[53; 62)
    MOD_KW@[53; 56) "mod"
    WHITESPACE@[56; 57) " "
    NAME@[57; 60)
      IDENT@[57; 60) "baz"
    SEMI@[60; 61) ";"
    WHITESPACE@[61; 62) "\n"
  TYPE_ALIAS_DEF@[62; 102)
    TYPE_KW@[62; 66) "type"
    WHITESPACE@[66; 67) " "
    NAME@[67; 79)
      IDENT@[67; 79) "ParserResult"
    TYPE_PARAM_LIST@[79; 83)
      L_ANGLE@[79; 80) "<"
      TYPE_PARAM@[80; 81)
        NAME@[80; 81)
          IDENT@[80; 81) "T"
      R_ANGLE@[81; 82) ">"
      WHITESPACE@[82; 83) " "
    EQ@[83; 84) "="
    WHITESPACE@[84; 85) " "
    IDENT_TYPE@[85; 98)
      IDENT@[85; 91) "Result"
      TYPE_PARAM_LIST@[91; 98)
        L_ANGLE@[91; 92) "<"
        TYPE_PARAM@[92; 93)
          NAME@[92; 93)
            IDENT@[92; 93) "T"
        COMMA@[93; 94) ","
        TYPE_PARAM@[94; 97)
          NAME@[94; 97)
            IDENT@[94; 97) "i32"
        R_ANGLE@[97; 98) ">"
    SEMI@[98; 99) ";"
    WHITESPACE@[99; 102) "\n\n\n"
  FN_DEF@[102; 128)
    VISIBILITY@[102; 109)
      EXPORT_KW@[102; 108) "export"
      WHITESPACE@[108; 109) " "
    FN_KW@[109; 111) "fn"
    WHITESPACE@[111; 112) " "
    NAME@[112; 116)
      IDENT@[112; 116) "main"
    PARAM_LIST@[116; 119)
      L_PAREN@[116; 117) "("
      R_PAREN@[117; 118) ")"
      WHITESPACE@[118; 119) " "
    BLOCK_EXPR@[119; 128)
      BLOCK@[119; 128)
        L_CURLY@[119; 120) "{"
        WHITESPACE@[120; 126) "\n    \n"
        R_CURLY@[126; 127) "}"
        WHITESPACE@[127; 128) "\n"

"mod bar;\n"
"mod foo;\n"
"mod baz;\n"
"mod baz;\n"
ModuleGraph { nodes: {}, edges: {} }
"mod bar;\n\n"
ModuleGraph { nodes: {FileId(2), FileId(1)}, edges: {FileId(2): {NameId(1): FileId(1)}, FileId(1): {}} }
