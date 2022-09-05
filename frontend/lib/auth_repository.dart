import 'dart:async';
import 'dart:convert';
import 'package:dio/dio.dart';
import 'package:dio_http2_adapter/dio_http2_adapter.dart';
import 'package:rssmailer/account_repository.dart';
import 'package:rssmailer/model/account.dart';

class AuthRepository {
  final _ctrl = StreamController<AuthRepoStatus>();

  Stream<AuthRepoStatus> get status async* {
    yield const AuthRepoStatus.unauth();
    yield* _ctrl.stream;
  }

  Future<String> login(
      {required String email, required String password}) async {
    final dio = Dio()
      ..options.baseUrl = kRssMailerURL
      ..interceptors.add(LogInterceptor())
      ..httpClientAdapter = Http2Adapter(
        ConnectionManager(
          idleTimeout: 10000,
          onClientCreate: (_, config) => config.onBadCertificate = (_) => true,
        ),
      );
    final loginReq = LoginRequest(
        email: email, provider: AuthProvider.plumage, authKey: password);
    final response = await dio.post('/login', data: jsonEncode(loginReq));

    final data = response.data;
    if (data == null) {
      return '';
    }
    final loginResp = LoginResponse.fromJson(data);

    _ctrl.add(AuthRepoStatus.auth(loginResp.token));
    return loginResp.token;
  }

  int logout(Account account) {
    _ctrl.add(const AuthRepoStatus.unauth());
    return 0;
  }

  dispose() {
    _ctrl.close();
  }
}
