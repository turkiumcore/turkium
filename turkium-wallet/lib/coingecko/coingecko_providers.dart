import 'package:decimal/decimal.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../core/core_providers.dart';
import '../settings/settings_providers.dart';
import 'coingecko_price_notifier.dart';
import 'coingecko_repository.dart';
import 'coingecko_types.dart';

final _turkiumPriceCacheProvider =
    StateNotifierProvider<CoinGeckoPriceNotifier, CoinGeckoPrice>((ref) {
  final repository = ref.watch(settingsRepositoryProvider);
  return CoinGeckoPriceNotifier(repository);
});

final _turkiumPriceRemoteProvider =
    FutureProvider.autoDispose<CoinGeckoPrice>((ref) async {
  ref.watch(remoteRefreshProvider);
  ref.watch(timeProvider);

  final currency = ref.watch(currencyProvider);
  final fiat = currency.name.toLowerCase();

  final log = ref.read(loggerProvider);
  final cached = ref.read(_turkiumPriceCacheProvider);

  // 60 seconds
  final maxCacheAge = 60 * 1000;
  final timestamp = DateTime.now().millisecondsSinceEpoch;
  if (cached.currency == currency.currency &&
      timestamp - cached.timestamp < maxCacheAge) {
    log.d('Using cached CoinGecko exchange rates');
    return cached;
  }

  try {
    var price = await getCoinGeckoApiPrice(fiat);
    // fallback to Turkium API if CoinGecko API fails
    if (price == null) {
      price = await getTurkiumApiPrice(fiat);
    }
    if (price == null) {
      throw Exception('Failed to fetch remote exchange rate');
    }

    return CoinGeckoPrice(
      currency: currency.currency,
      price: Decimal.parse(price.toString()),
      timestamp: timestamp,
    );
  } catch (e, st) {
    log.e('Failed to fetch KAS exchange rate', error: e, stackTrace: st);
    if (cached.currency == currency.currency) {
      return cached;
    }
    return CoinGeckoPrice(
      currency: currency.currency,
      price: Decimal.zero,
      timestamp: timestamp,
    );
  }
});

final coingeckoTurkiumPriceProvider = Provider.autoDispose((ref) {
  final cache = ref.watch(_turkiumPriceCacheProvider.notifier);
  final remote = ref.watch(_turkiumPriceRemoteProvider);

  remote.whenOrNull(data: (data) {
    Future.microtask(() => cache.updatePrice(data));
  });

  return remote.asData?.value ?? cache.price;
});
