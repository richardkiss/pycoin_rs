import time

from pycoin_rs import utils
import bitcoin

def decode_p2w(script_pubkey):
    try:
        bech32 = bitcoin.bech32.CBech32Data.from_bytes(0, script_pubkey[2:22])
        return str(bech32), None
    except TypeError as e:
        raise Exception('bech32 decoding error')


def inverse_hash_py(hashstring):
    return ''.join([hashstring[i:i+2][::-1] for i in range(0, len(hashstring), 2)])[::-1]


def test_inverse_hash():
    h = "b5276739a3e0f32147bd4a921f936c6013dee4a5ca426ee2de868810b068ec0d"
    assert utils.inverse_hash(h) == "0dec68b0108886dee26e42caa5e4de13606c931f924abd4721f3e0a3396727b5"

    #iterations = 1000000
    iterations = 10

    start_time = time.time()
    for i in range(iterations):
        inverse_hash_py(h)
    python_duration = time.time() - start_time
    print(f'{iterations} inverse hash with python: {python_duration}s')

    start_time = time.time()
    for i in range(iterations):
        utils.inverse_hash(h)
    rust_duration = time.time() - start_time
    print(f'{iterations} inverse hash with rust: {rust_duration}s')

    assert python_duration > rust_duration


def test_script_to_asm():
    asm = utils.script_to_asm(b'v\xa9\x14H8\xd8\xb3X\x8cL{\xa7\xc1\xd0o\x86n\x9b79\xc607\x88\xac')
    assert asm == [b'v', b'\xa9', b'H8\xd8\xb3X\x8cL{\xa7\xc1\xd0o\x86n\x9b79\xc607', b'\x88', b'\xac']
    """ [
        'OP_DUP',
        'OP_HASH160',
        b'H8\xd8\xb3X\x8cL{\xa7\xc1\xd0o\x86n\x9b79\xc607',
        'OP_EQUALVERIFY',
        'OP_CHECKSIG'
    ] """

def test_decode_p2w():
    script_pubkey = b'\x00\x14u\x1ev\xe8\x19\x91\x96\xd4T\x94\x1cE\xd1\xb3\xa3#\xf1C;\xd6'
    bech32 = utils.script_to_address(script_pubkey, 'testnet')
    assert bech32 == "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx"

    bech32 = utils.script_to_address(script_pubkey, 'mainnet')
    assert bech32 == "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
