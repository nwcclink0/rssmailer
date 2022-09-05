import 'package:equatable/equatable.dart';
import 'package:bloc/bloc.dart';
import 'package:formz/formz.dart';
import 'package:rssmailer/auth_repository.dart';
import 'package:rssmailer/model/account.dart';

class LoginEvent extends Equatable {
  const LoginEvent();
  @override
  List<Object?> get props => [];
}

class LoginEmailChanged extends LoginEvent {
  final String email;
  const LoginEmailChanged({required this.email});

  @override
  List<Object?> get props => [email];
}

class LoginPasswordChanged extends LoginEvent {
  final String password;
  const LoginPasswordChanged({required this.password});

  @override
  List<Object?> get props => [password];
}

class LoginSubmitted extends LoginEvent {
  final String email;
  final String password;
  const LoginSubmitted({required this.email, required this.password});

  @override
  List<Object?> get props => [email, password];
}

class LoginState extends Equatable {
  const LoginState(
      {this.status = FormzStatus.pure,
      this.emailAddress = const EmailAddress.pure(),
      this.password = const Password.pure()});

  final FormzStatus status;
  final EmailAddress emailAddress;
  final Password password;

  LoginState copyWith(
      {FormzStatus? status, EmailAddress? emailAddress, Password? password}) {
    return LoginState(
        status: status ?? this.status,
        emailAddress: emailAddress ?? this.emailAddress,
        password: password ?? this.password);
  }

  @override
  List<Object?> get props => [status, emailAddress, password];
}

class LoginBloc extends Bloc<LoginEvent, LoginState> {
  final AuthRepository _authRepository;
  LoginBloc({required AuthRepository authRepository})
      : _authRepository = authRepository,
        super(const LoginState()) {
    on<LoginEmailChanged>(_onEmailChanged);
    on<LoginPasswordChanged>(_onPasswordChanged);
    on<LoginSubmitted>(_onSubmitted);
  }

  _onEmailChanged(LoginEmailChanged event, Emitter<LoginState> emitter) {
    final email = EmailAddress.dirty(event.email);
    emitter(state.copyWith(
        emailAddress: email, status: Formz.validate([state.password, email])));
  }

  _onPasswordChanged(LoginPasswordChanged event, Emitter<LoginState> emitter) {
    final password = Password.dirty(event.password);
    emitter(state.copyWith(
        password: password,
        status: Formz.validate([state.emailAddress, password])));
  }

  _onSubmitted(LoginSubmitted event, Emitter<LoginState> emitter) async {
    if (state.status.isValid) {
      emitter(state.copyWith(status: FormzStatus.submissionInProgress));
      try {
        String token = await _authRepository.login(
            email: state.emailAddress.value, password: state.password.value);
        if (token.isEmpty) {
          emitter(state.copyWith(status: FormzStatus.submissionSuccess));
        } else {
          emitter(state.copyWith(status: FormzStatus.submissionFailure));
        }
      } catch (e) {
        print("login failed: " + e.toString());
        emitter(state.copyWith(status: FormzStatus.submissionFailure));
      }
    }
  }
}
