import 'dart:math';

import 'package:flutter_test/flutter_test.dart';
import 'package:rssmailer/account_repository.dart';
import 'package:rssmailer/auth_repository.dart';
import 'package:rssmailer/model/account.dart';

const String email = 'yuantingwei@arcsparrow.com';
const String password = 'rssmailer';
const String nickname = 'yt';

void main() {
  late AccountRepository accountRepository;
  late AuthRepository authRepository;

  setUp(() {
    authRepository = AuthRepository();
    accountRepository = AccountRepository();
  });

  group('create account ', () {
    String token = '';
    late Account account;
    test('index', () async {
      await accountRepository.index();
    });
    test('create account', () async {
      account =
          await accountRepository.createAccount(email, password, nickname);
      expect(account.id.isNotEmpty, true);
    });

    test('create account failed', () async {
      account = await accountRepository.createAccount('', password, nickname);
      expect(account.id.isEmpty, true);
    });

    test('login account', () async {
      token = await authRepository.login(email: email, password: password);
    });

    test('get account', () async {
      if (token.isEmpty) {}
      final ret = await accountRepository.deleteAccount(token);
      expect(ret, 0);
    });
  });
  group('verify account', () {
    String verify_code = '';
  });

  group('auth login', () {
    test('create account', () {});
  });
}
