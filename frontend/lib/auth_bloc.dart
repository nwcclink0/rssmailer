import 'dart:async';

import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rssmailer/account_repository.dart';
import 'package:rssmailer/auth_repository.dart';
import 'package:rssmailer/model/account.dart';

abstract class AuthEvent extends Equatable {
  const AuthEvent();

  @override
  List<Object> get props => [];
}

class AuthRepoStatusChanged extends AuthEvent {
  final AuthRepoStatus repoStatus;
  const AuthRepoStatusChanged({required this.repoStatus});
  @override
  List<Object> get props => [repoStatus];
}

class AuthLogoutRequested extends AuthEvent {
  final String token;

  const AuthLogoutRequested({required this.token});
  @override
  List<Object> get props => [token];
}

class AuthState extends Equatable {
  final AuthStatus status;
  final Account account;

  const AuthState._({
    this.status = AuthStatus.unauth,
    this.account = Account.empty,
  });

  const AuthState.unknown() : this._();
  const AuthState.auth(Account account)
      : this._(status: AuthStatus.auth, account: account);
  const AuthState.unauth()
      : this._(status: AuthStatus.unauth, account: Account.empty);

  @override
  List<Object?> get props => [status, account];
}

class AuthBloc extends Bloc<AuthEvent, AuthState> {
  final AuthRepository _authRepository;
  final AccountRepository _accountRepository;
  late StreamSubscription<AuthRepoStatus> _authStreamSubscription;

  AuthBloc(
      {required AuthRepository authRepository,
      required AccountRepository accountRepository})
      : _authRepository = authRepository,
        _accountRepository = accountRepository,
        super(const AuthState.unknown()) {
    on<AuthRepoStatusChanged>(_onAuthStatusChanged);
    on<AuthLogoutRequested>(_onAuthLogoutRequested);
    _authStreamSubscription = _authRepository.status.listen((repoStatus) {
      add(AuthRepoStatusChanged(repoStatus: repoStatus));
    });
  }
  @override
  Future<void> close() {
    _authStreamSubscription.cancel();
    _authRepository.dispose();
    return super.close();
  }

  void _onAuthStatusChanged(
      AuthRepoStatusChanged event, Emitter<AuthState> emit) async {
    switch (event.repoStatus.status) {
      case AuthStatus.unauth:
        return emit(const AuthState.unauth());
      case AuthStatus.auth:
      // final account =
      //     await _accountRepository.getAccount(event.repoStatus.token);
      // if (account != null) {
      //   return emit(AuthState.auth(account));
      // } else {
      //   return emit(const AuthState.unknown());
      // }
      default:
        return emit(const AuthState.unknown());
    }
  }

  void _onAuthLogoutRequested(
      AuthLogoutRequested event, Emitter<AuthState> emit) async {}
}
