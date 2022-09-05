import 'dart:convert';
import 'package:dio_http2_adapter/dio_http2_adapter.dart';
import 'package:jwt_decoder/jwt_decoder.dart';

import 'model/account.dart';
import 'package:dio/dio.dart';

const kRssMailerURL = 'https://127.0.0.1:8443';

class AccountRepository {
  Account? _account;

  Future<void> index() async {
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          // onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
          onClientCreate: (uri, config) async {
            config.onBadCertificate = (_) {
              return true;
            };
          },
        ),
      );
    final response = await dio.get('/index.html');
    final data = response.data ?? '';
    print('data: ' + data);
  }

  // Future<void> index1() async {
  //   final client = HttpClient();
  //   client.badCertificateCallback =
  //       (X509Certificate cert, String host, int port) {
  //     return true;
  //   };
  //   final HttpClientRequest request =
  //       await client.getUrl(Uri.parse('https://127.0.0.1:8443/index.html'));
  //   final response = await request.close();
  //   String reply = await response.transform(utf8.decoder).join();
  //   print("reply: " + reply);
  // }

  Future<Account> createAccount(
      String email, String password, String nickname) async {
    if (email.isEmpty || password.isEmpty || nickname.isEmpty) {
      return Account.empty;
    }
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (uri, config) async {
            config.onBadCertificate = (_) {
              return true;
            };
          },
        ),
      );
    final response = await dio.post('/account/add',
        data: jsonEncode(CreateAccountRequest(
            email: email, password: password, nickname: nickname)));

    final data = response.data;
    if (data == null) {
      return Account.empty;
    }
    final account = Account.fromJson(data);
    return account;
  }

  Future<int> deleteAccount(String token) async {
    if (token.isEmpty) {
      return 1;
    }

    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
        ),
      )
      ..options.headers = {'x-csrf-token': token};

    final jwt = JwtDecoder.decode(token);
    final id = jwt['id'];
    final response = await dio.post('/account/' + id + "/delete");
    final data = response.data;
    if (data == null) {
      return 1;
    }
    final accountResp = AccountResponse.fromJson(data);
    if (accountResp.status != 0) {
      return 1;
    }
    return 0;
  }

  Future<Account?> getAccount(String token) async {
    if (_account != null) return _account;
    if (JwtDecoder.isExpired(token)) {
      Account.empty;
    }
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
        ),
      )
      ..options.headers = {'x-csrf-token': token};

    final jwt = JwtDecoder.decode(token);
    final userId = jwt['id'];

    final response = await dio.post('/account/' + userId);
    final data = response.data ?? '';
    if (data.isEmpty) {
      return Account.empty;
    }
    final accountResp = AccountResponse.fromJson(data);
    if (accountResp.status != 0) {
      return Account.empty;
    }
    return accountResp.account;
  }

  Future<int> sendVerifyCode(String email) async {
    if (email.isEmpty) {
      return 1;
    }
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
        ),
      );
    final verifyReq = AccountVerifyEmailRequest(email: email);
    final response =
        await dio.post('/send_verify_email_code', data: jsonEncode(verifyReq));
    final data = response.data;
    if (data == null) {
      return 1;
    }
    final verifyResp = AccountVerifyEmailResponse.fromJson(data);
    if (verifyResp.status != 0) {
      return 1;
    }
    return 0;
  }

  Dio getDio() {
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
        ),
      );
    return dio;
  }
}
