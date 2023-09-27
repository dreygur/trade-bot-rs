## To test

**Fork Mainnet**

```sh
ganache -a 10 -m "truly hood nurse paper surprise visual stable slender liberty moral salute cloth" -f -u "0xdEAD000000000000000042069420694206942069" -u "0x7cB769025F9CCFdf2DF576bf848479Dabf8BF195" -p 7545
```

**Transfer WETH to test Wallet**

```sh
node scripts/transfer.js
```

**Manipulate price to trigger the bot**

```sh
node scripts/manipulatePrice.js
```
