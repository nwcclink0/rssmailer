import 'dart:ffi';

import 'package:equatable/equatable.dart';
import 'package:formz/formz.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:rssmailer/account_repository.dart';

part 'account.g.dart';

enum AuthStatus {
  unknown,
  auth,
  unauth,
}

class AuthProvider {
  static const plumage = 0;
  static const google = 1;
}

class AuthRepoStatus {
  final AuthStatus status;
  final String token;

  const AuthRepoStatus._({
    this.status = AuthStatus.unauth,
    this.token = '',
  });

  const AuthRepoStatus.unknown() : this._();
  const AuthRepoStatus.auth(String token)
      : this._(status: AuthStatus.auth, token: token);
  const AuthRepoStatus.unauth() : this._(status: AuthStatus.unauth, token: '');
}

@JsonSerializable()
class Account extends Equatable {
  final String id;
  final String nickname;
  final String email;

  const Account({
    required this.id,
    required this.nickname,
    required this.email,
  });

  static const empty = Account(nickname: '', email: '', id: '');

  @override
  List<Object?> get props => [id, email, nickname];

  factory Account.fromJson(Map<String, dynamic> json) =>
      _$AccountFromJson(json);
  Map<String, dynamic> toJson() => _$AccountToJson(this);
}

enum EmailAddressValidationError { empty, incorrectAddress }

class EmailAddress extends FormzInput<String, EmailAddressValidationError> {
  const EmailAddress.pure() : super.pure('');
  const EmailAddress.dirty([String value = '']) : super.dirty(value);

  @override
  EmailAddressValidationError? validator(String value) {
    if (value.isEmpty) {
      return EmailAddressValidationError.empty;
    }
    final reg = RegExp(r'^[a-zA-Z0-9`\-=[\];,.~!@#$^&*()_+{}|:<>?]*$');
    if (reg.hasMatch(value)) {
      return null;
    } else {
      return EmailAddressValidationError.incorrectAddress;
    }
  }
}

enum PasswordValidationError { empty }

class Password extends FormzInput<String, PasswordValidationError> {
  const Password.pure() : super.pure('');
  const Password.dirty([String value = '']) : super.dirty(value);

  @override
  PasswordValidationError? validator(String value) {
    if (value.isEmpty) {
      return PasswordValidationError.empty;
    }
    return null;
  }
}

@JsonSerializable()
class AccountResponse {
  final Account account;
  final int status;

  const AccountResponse({required this.account, required this.status});

  factory AccountResponse.fromJson(Map<String, dynamic> json) =>
      _$AccountResponseFromJson(json);
  Map<String, dynamic> toJson() => _$AccountResponseToJson(this);
}

@JsonSerializable()
class LoginRequest {
  final String email;
  final int provider;

  @JsonKey(name: "auth_key")
  final String authKey;

  const LoginRequest(
      {required this.email, required this.provider, required this.authKey});

  factory LoginRequest.fromJson(Map<String, dynamic> json) =>
      _$LoginRequestFromJson(json);

  Map<String, dynamic> toJson() => _$LoginRequestToJson(this);
}

@JsonSerializable()
class LoginResponse {
  final String token;
  final int status;

  const LoginResponse({required this.token, required this.status});

  factory LoginResponse.fromJson(Map<String, dynamic> json) =>
      _$LoginResponseFromJson(json);
  Map<String, dynamic> toJson() => _$LoginResponseToJson(this);
}

@JsonSerializable()
class CreateAccountRequest {
  final String email;
  final String password;
  final String nickname;

  const CreateAccountRequest(
      {required this.email, required this.password, required this.nickname});

  factory CreateAccountRequest.fromJson(Map<String, dynamic> json) =>
      _$CreateAccountRequestFromJson(json);
  Map<String, dynamic> toJson() => _$CreateAccountRequestToJson(this);
}

@JsonSerializable()
class AccountVerifyEmailRequest {
  final String email;
  const AccountVerifyEmailRequest({required this.email});

  factory AccountVerifyEmailRequest.fromJson(Map<String, dynamic> json) =>
      _$AccountVerifyEmailRequestFromJson(json);
  Map<String, dynamic> toJson() => _$AccountVerifyEmailRequestToJson(this);
}

@JsonSerializable()
class AccountVerifyEmailResponse {
  final int status;
  const AccountVerifyEmailResponse({required this.status});

  factory AccountVerifyEmailResponse.fromJson(Map<String, dynamic> json) =>
      _$AccountVerifyEmailResponseFromJson(json);
  Map<String, dynamic> toJson() => _$AccountVerifyEmailResponseToJson(this);
}
