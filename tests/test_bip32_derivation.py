import time

from pycoin_rs import public_keys_for_xpub_str_and_paths, MAINNET


def test_bip32_derivation():
    XPUB = (
        "xpub661MyMwAqRbcFLDp2XRhusibkNn3gGtTS85mTCLMFzeuSxvsTKe"
        "vyS8c4SgHUCfoAwSWc6zY4TgSVAG9NGpfwb78yNZkcUxmavdRYCJqhLH"
    )

    start = time.time()
    t = public_keys_for_xpub_str_and_paths(XPUB, [f"m/{n}" for n in range(100000)])
    end = time.time()
    elapsed = end - start
    print(f"took {elapsed}")

    start = time.time()
    addresses = [_.p2pkh_address(MAINNET) for _ in t]
    end = time.time()
    elapsed = end - start
    print(f"took {elapsed}")

    # `ku P:1 -s 0 -a`
    assert str(addresses[0]) == "12zEBuFnmJwHMxs9GHV12WkZiBoDMUtYTP"

    # `ku P:1 -s 99999 -a`
    assert str(addresses[99999]) == "1A2vPRHSvZkeJ4Q1ZvLHiipcihPT1whYS9"

test_bip32_derivation()