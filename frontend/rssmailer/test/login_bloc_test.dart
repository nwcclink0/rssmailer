import 'package:bloc_test/bloc_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:formz/formz.dart';
import 'package:mocktail/mocktail.dart';
import 'package:rssmailer/account_repository.dart';
import 'package:rssmailer/auth_repository.dart';
import 'package:rssmailer/login_bloc.dart';
import 'package:rssmailer/model/account.dart';

class MockAuthRepository extends Mock implements AuthRepository {}

const String email = 'yuantingwei@arcsparrow.com';
const String password = 'rssmailer';

void main() {
  late LoginBloc loginBloc;
  late AuthRepository authRepo;
  late AccountRepository accountRepo;
  late Account account;
  setUp(() {
    accountRepo = AccountRepository();

    authRepo = MockAuthRepository();
    loginBloc = LoginBloc(authRepository: authRepo);
  });

  group('loginBloc', () {
    // test('create account', () async {
    //   account = await accountRepo.createAccount(email, password, password);
    //   expect(account.id.isEmpty, false);
    // });
    test('init login state', () {
      expect(loginBloc.state, const LoginState());
    });

    group('loginSubmitted', () {
      blocTest<LoginBloc, LoginState>('login success',
          build: () {
            when(() {
              return authRepo.login(email: email, password: password);
            }).thenAnswer((_) => Future.value('fake_token'));
            return loginBloc;
          },
          act: (bloc) {
            bloc
              ..add(const LoginEmailChanged(email: email))
              ..add(const LoginPasswordChanged(password: password))
              ..add(const LoginSubmitted(email: email, password: password));
          },
          expect: () => const <LoginState>[
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    status: FormzStatus.invalid),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.valid),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.submissionInProgress),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.submissionSuccess)
              ]);
      blocTest<LoginBloc, LoginState>('login failed',
          build: () {
            when(() {
              return authRepo.login(email: email, password: password);
            }).thenAnswer((_) => Future.value(''));
            return loginBloc;
          },
          expect: () => const <LoginState>[
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    status: FormzStatus.invalid),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.valid),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.submissionInProgress),
                LoginState(
                    emailAddress: EmailAddress.dirty(email),
                    password: Password.dirty(password),
                    status: FormzStatus.submissionFailure)
              ]);
      test('delete account', () async {});
    });

    group('Account', () {});
  });
}
