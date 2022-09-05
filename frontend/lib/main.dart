import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:rssmailer/account_repository.dart';
import 'package:rssmailer/auth_bloc.dart';
import 'package:rssmailer/auth_repository.dart';
import 'package:rssmailer/login_page.dart';
import 'package:rssmailer/model/account.dart';
import 'package:rssmailer/rsslist_page.dart';

void main() {
  final authRepo = AuthRepository();
  final accountRepo = AccountRepository();
  runApp(MyApp(
    authRepo: authRepo,
    accountRepo: accountRepo,
  ));
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key, required this.authRepo, required this.accountRepo})
      : super(key: key);
  final AuthRepository authRepo;
  final AccountRepository accountRepo;

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return RepositoryProvider.value(
      value: authRepo,
      child: BlocProvider(
        create: (_) =>
            AuthBloc(authRepository: authRepo, accountRepository: accountRepo),
        child: const AppView(),
      ),
    );
  }
}

class AppView extends StatefulWidget {
  const AppView({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return AppViewState();
  }
}

class AppViewState extends State<AppView> {
  final _navigatorKey = GlobalKey<NavigatorState>();

  NavigatorState get _navigator => _navigatorKey.currentState!;

  @override
  Widget build(BuildContext context) {
    return ScreenUtilInit(
        designSize: const Size(1194, 834),
        builder: () => MaterialApp(
              builder: (context, child) {
                return BlocListener<AuthBloc, AuthState>(
                  listener: (context, state) {
                    switch (state.status) {
                      case AuthStatus.unauth:
                        _navigator.pushAndRemoveUntil<void>(
                            LoginPage.route(), (route) => false);
                        break;
                      case AuthStatus.auth:
                        _navigator.pushAndRemoveUntil<void>(
                            RssListPage.route(), (route) => false);
                        break;
                      default:
                        break;
                    }
                  },
                  child: child,
                );
              },
            ));
  }
}
