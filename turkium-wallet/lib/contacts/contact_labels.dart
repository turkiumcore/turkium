import 'contact.dart';

const kTurkiumDevFundAddress =
    'turkium:precqv0krj3r6uyyfa36ga7s0u9jct0v4wg8ctsfde2gkrsgwgw8jgxfzfc98';

const kCommunityMarketingFundAddress =
    'turkium:qpyr8yp7jmantyaatyqtpwuzv3fcthe2r7jz6n55nl4hdd0288e8jljunnqgv';

const kTurkiumTeamAddress =
    'turkium:qp0qnczz5jc8vpujuyqrjw5eey73paqfgxks7505pqdz9eqwmwftxg44gu00z';

const kDonationAddresses = const [
  Contact(
    name: 'Turkium Dev Fund',
    address: kTurkiumDevFundAddress,
  ),
  Contact(
    name: 'Community Marketing Fund',
    address: kCommunityMarketingFundAddress,
  ),
  Contact(
    name: 'Turkium Team',
    address: kTurkiumTeamAddress,
  ),
];

final labeledAddresses = Map.fromEntries(
  kDonationAddresses.map((e) => MapEntry(e.address, e)),
);
