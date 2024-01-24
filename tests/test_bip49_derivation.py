import time

from pycoin_rs import public_keys_for_xpub_str_and_paths, MAINNET


def test_bip49_derivation():
    # Extended Public Key (XPUB) for BIP49 derivation
    XPUB = (
        "ypub6QqdH2c5z7967pFbK9GYKHsEDSV1vk6HrDU5SymGSwWgozSE9nW4Qn3spo8yiq44BTGrg3xe7SCCkYuidNUvdjbJHFsc3f6ohb1sV84EwCm"
    )

    # Measure the time taken to derive public keys
    start = time.time()
    t = public_keys_for_xpub_str_and_paths(XPUB, [f"m/0/{n}" for n in range(100000)])
    end = time.time()
    elapsed = end - start
    print(f"Deriving public keys took {elapsed} seconds")

    # Measure the time taken to generate P2SHWPKH addresses
    start = time.time()
    addresses_p2shwpkh = [_.p2shwpkh_address(MAINNET) for _ in t]
    end = time.time()
    elapsed = end - start
    print(f"Generating P2SHWPKH addresses took {elapsed} seconds")

    # Validate the first and last generated addresses
    assert str(addresses_p2shwpkh[0]) == "3NZhMU2ubr52jTaHLqdoeP5sNLXaZgREjU"
    assert str(addresses_p2shwpkh[99999]) == "3JH7BTQ82ycAfAhzx45ti1sFhC33mnqNnU"


# Run the test function
test_bip49_derivation()
