/**
 *
 * sha256 :
 * hash 100 u8  : 23619 CU
 * hash 200 u8  : 36998 CU
 * hash 300 u8  : 43723 CU
 * hash 400 u8  : 57106 CU
 * hash 500 u8  : 63862 CU
 * hash 1000 u8 : 117534 CU
 * ===> 100 CU / byte hashed ==> max ~120kb can be hashed on chain with sha256
 * ==> alpha4 ~ 800kb ==> 80M CU
 *
 *
 * sha1 :
 * hash 1000u8 : 51401 CU
 * hash 2000u8: 95777 CU
 * ===> 45 CU / byte hashed ==> max ~240kb can be hashed on chain with sha1
 *
 * sha1  one over 4:
 * hash 2000u8 : 36573 CU
 */
