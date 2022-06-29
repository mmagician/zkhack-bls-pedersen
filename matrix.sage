# scalar field of the curve = Fr
field_modulus = 52435875175126190479447740508185965837690552500527637822603658699938581184513
# field_modulus = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
F = GF(field_modulus)

load('/Users/marcin/personal/zkhack-bls-pedersen/M256.sage')
load('/Users/marcin/personal/zkhack-bls-pedersen/my_msg_hash.sage')

ms_vec_f = list(map(lambda x: F(x), ms_vec_binary))
M = matrix(256, 256, ms_vec_f)

inv = M.inverse()

B_vec = list(map(lambda x: F(x), my_msg_hash))
B = matrix(256, 1, B_vec)

x = inv * B
