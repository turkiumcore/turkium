import 'bip32/bip32.dart';

const String kTurkiumNetworkMainnet = 'mainnet';
const String kTurkiumNetworkTestnet = 'testnet';
const String kTurkiumNetworkSimnet = 'simnet';
const String kTurkiumNetworkDevnet = 'devnet';

const String kTurkiumNetworkIdMainnet = '$kTurkiumNetworkMainnet';
const String kTurkiumNetworkIdTestnet10 = '$kTurkiumNetworkTestnet-10';
const String kTurkiumNetworkIdTestnet11 = '$kTurkiumNetworkTestnet-11';
const String kTurkiumNetworkIdSimnet = '$kTurkiumNetworkSimnet';
const String kTurkiumNetworkIdDevnet = '$kTurkiumNetworkDevnet';

const int kMainnetRpcPort = 16110;
const int kTestnetPpcPort = 16210;
const int kSimnetRpcPort = 16510;
const int kDevnetRpcPort = 16610;

enum TurkiumNetwork {
  mainnet,
  testnet,
  devnet,
  simnet;

  static TurkiumNetwork? tryParse(String network) {
    return switch (network) {
      kTurkiumNetworkMainnet => TurkiumNetwork.mainnet,
      kTurkiumNetworkTestnet => TurkiumNetwork.testnet,
      kTurkiumNetworkSimnet => TurkiumNetwork.simnet,
      kTurkiumNetworkDevnet => TurkiumNetwork.devnet,
      _ => null,
    };
  }

  String idWithSuffix([String suffix = '']) {
    if (suffix.isNotEmpty) {
      return name + '-$suffix';
    }
    return name;
  }

  int get defaultRpcPort => switch (this) {
        TurkiumNetwork.mainnet => kMainnetRpcPort,
        TurkiumNetwork.testnet => kTestnetPpcPort,
        TurkiumNetwork.simnet => kSimnetRpcPort,
        TurkiumNetwork.devnet => kDevnetRpcPort
      };
}

TurkiumNetwork networkForPort(int port) {
  switch (port) {
    case kMainnetRpcPort:
      return TurkiumNetwork.mainnet;
    case kTestnetPpcPort:
      return TurkiumNetwork.testnet;
    case kSimnetRpcPort:
      return TurkiumNetwork.simnet;
    case kDevnetRpcPort:
      return TurkiumNetwork.devnet;
    default:
      return TurkiumNetwork.mainnet;
  }
}

TurkiumNetwork networkForKpub(String kpub) {
  return switch (kpub.substring(0, 4)) {
    'kpub' => TurkiumNetwork.mainnet,
    'ktub' => TurkiumNetwork.testnet,
    'ksub' => TurkiumNetwork.simnet,
    'kdub' => TurkiumNetwork.devnet,
    _ => TurkiumNetwork.mainnet,
  };
}

NetworkType networkTypeForNetwork(TurkiumNetwork network) {
  switch (network) {
    case TurkiumNetwork.mainnet:
      return turkiumMainnet;
    case TurkiumNetwork.testnet:
      return turkiumTestnet;
    case TurkiumNetwork.devnet:
      return turkiumDevnet;
    case TurkiumNetwork.simnet:
      return turkiumSimnet;
  }
}

final turkiumMainnet = NetworkType(
  messagePrefix: 'Turkium Signed Message:\n',
  bech32: 'turkium',
  bip32: Bip32Type(
    public: 0x038f332e,
    private: 0x038f2ef4,
  ),
  wif: 0x80,
  pubKeyHash: 0x00,
  scriptHash: 0x05,
  opreturnSize: 80,
);

final turkiumTestnet = NetworkType(
  messagePrefix: 'Turkium Signed Message:\n',
  bech32: 'turkiumtest',
  wif: 0xef,
  bip32: Bip32Type(
    public: 0x0390a241,
    private: 0x03909e07,
  ),
  pubKeyHash: 0x00,
  scriptHash: 0x05,
  opreturnSize: 80,
);

final turkiumSimnet = NetworkType(
  messagePrefix: 'Turkium Signed Message:\n',
  bech32: 'turkiumsim',
  wif: 0x64,
  bip32: Bip32Type(
    public: 0x0390467d,
    private: 0x03904242,
  ),
  pubKeyHash: 0x00,
  scriptHash: 0x05,
  opreturnSize: 80,
);

final turkiumDevnet = NetworkType(
  messagePrefix: 'Turkium Signed Message:\n',
  bech32: 'turkiumdev',
  wif: 0xef,
  bip32: Bip32Type(
    public: 0x038b41ba,
    private: 0x038b3d80,
  ),
  pubKeyHash: 0x00,
  scriptHash: 0x05,
  opreturnSize: 80,
);
