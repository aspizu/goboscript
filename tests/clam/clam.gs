costumes "blank.svg";

onflag {
    CLAM_Init 1024, "root";
    CLAM_StoreNode "message";
    CLAM_EnterLocallyIndex 1;
    CLAM_StoreData "Hello", "a";
    CLAM_Exit;
    CLAM_StoreNode "a";
    CLAM_EnterLocallyKey "a";
    CLAM_StoreData "abb", 1;
    CLAM_StoreData "abbreviation", 2;
    CLAM_FetchDataOfKey 1, 0;
    CLAM_Exit;
}
