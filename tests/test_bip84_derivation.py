import time

from pycoin_rs import public_keys_for_xpub_str_and_paths, MAINNET


def test_bip84_derivation():
    # Extended Public Key (XPUB) for BIP84 derivation
    XPUB = (
        "zpub6nBMtUezHUxR5iQc9rqBATiJk7DZmcETFKud1UMAJ4Pi3rnDMXpUNzATfAwk8rzjCb5ysvPWPm14k85qSRVxzcGkR9YxXWzxK7uAVHPh8nc"
    )

    # Measure the time taken to derive public keys
    start = time.time()
    t = public_keys_for_xpub_str_and_paths(XPUB, [f"m/0/{n}" for n in range(100000)])
    end = time.time()
    elapsed = end - start
    print(f"Deriving public keys took {elapsed} seconds")

    # Measure the time taken to generate P2WPKH addresses
    start = time.time()
    addresses_p2wpkh = [_.p2wpkh_address(MAINNET) for _ in t]
    end = time.time()
    elapsed = end - start
    print(f"Generating P2WPKH addresses took {elapsed} seconds")  

    # Print the first and last generated addresses
    print(addresses_p2wpkh[0], addresses_p2wpkh[99999])
    
    # Validate the first and last generated addresses
    assert str(addresses_p2wpkh[0]) == "bc1qdwqj4g57vklgv38e9ym96evn8cym7krclzh6ys"
    assert str(addresses_p2wpkh[99999]) == "bc1qaez7srh8zs87ckcct7m2q0kd80re6j8dger9mj"
    

# Run the test function
test_bip84_derivation()
