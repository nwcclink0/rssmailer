import 'package:email_validator/email_validator.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:rssmailer/app_style.dart';
import 'package:rssmailer/common_style.dart';

class LoginPage extends StatefulWidget {
  LoginPage({Key? key}) : super(key: key);

  static Route route() {
    return MaterialPageRoute(builder: (_) {
      return LoginPage();
    });
  }

  @override
  State<StatefulWidget> createState() {
    return LoginPageState();
  }
}

class LoginPageState extends State<LoginPage> {
  final TextEditingController emailCtr = TextEditingController();
  final TextEditingController passwordCtr = TextEditingController();
  bool rememberMe = false;
  @override
  Widget build(BuildContext context) {
    final scaffold = Scaffold(
        body: SingleChildScrollView(
            child: ColoredBox(
                color: AppColors.paleGrey,
                child: Center(
                    child: SizedBox(
                        width: 282.r,
                        height: ScreenUtil().screenHeight,
                        child: Padding(
                          padding: EdgeInsets.only(top: 20.r, bottom: 20.r),
                          child: Column(
                            children: [
                              Text(
                                "RSS-Mailer",
                                style: TextStyle(
                                  color: AppColors.darkSkyBlue,
                                  fontSize: 48.sp,
                                  fontFamily: "Ubuntu",
                                  fontWeight: FontWeight.w700,
                                ),
                              ),
                              Padding(padding: EdgeInsets.only(top: 71.r)),
                              EmailTextField(ctrl: emailCtr),
                              Padding(padding: EdgeInsets.only(top: 24.r)),
                              PasswordTextField(
                                  key: const Key("password"),
                                  ctr: passwordCtr,
                                  isConfirmPassword: false),
                              Row(
                                mainAxisAlignment: MainAxisAlignment.start,
                                children: [
                                  Switch(
                                      activeColor: AppColors.darkSkyBlue,
                                      value: rememberMe,
                                      onChanged: (newVal) {
                                        setState(() {
                                          rememberMe = newVal;
                                        });
                                      }),
                                  Padding(padding: EdgeInsets.only(left: 8.r)),
                                  Text(
                                    "Remember Me",
                                    style: TextStyle(
                                        color: AppColors.greyBlue,
                                        fontWeight: FontWeight.w400,
                                        fontFamily: "Ubuntu",
                                        fontStyle: FontStyle.normal,
                                        fontSize: 12.0.sp),
                                  )
                                ],
                              ),
                              Padding(padding: EdgeInsets.only(top: 28.r)),
                              ConfirmBtn(
                                  key: const Key("loginConfirm"),
                                  title: "Login",
                                  onPress: () {},
                                  disable: false),
                              Padding(padding: EdgeInsets.only(top: 16.r)),
                              RichText(
                                  text: TextSpan(children: [
                                TextSpan(
                                    style: TextStyle(
                                        color: AppColors.greyBlue,
                                        fontWeight: FontWeight.w400,
                                        fontFamily: "Ubuntu",
                                        fontStyle: FontStyle.normal,
                                        fontSize: 12.0.sp),
                                    text: "Do you want to "),
                                TextSpan(
                                    style: TextStyle(
                                        color: AppColors.greyBlue,
                                        fontWeight: FontWeight.w700,
                                        fontFamily: "Ubuntu",
                                        fontStyle: FontStyle.normal,
                                        fontSize: 12.0.sp),
                                    text: "create account ",
                                    recognizer: TapGestureRecognizer()
                                      ..onTap = () => {}),
                                TextSpan(
                                    style: TextStyle(
                                        color: AppColors.greyBlue,
                                        fontWeight: FontWeight.w400,
                                        fontFamily: "Ubuntu",
                                        fontStyle: FontStyle.normal,
                                        fontSize: 12.0.sp),
                                    text: "? or "),
                                TextSpan(
                                    style: TextStyle(
                                        color: AppColors.greyBlue,
                                        fontWeight: FontWeight.w700,
                                        fontFamily: "Ubuntu",
                                        fontStyle: FontStyle.normal,
                                        fontSize: 12.0.sp),
                                    text: "reset password",
                                    recognizer: TapGestureRecognizer()
                                      ..onTap = () => {})
                              ]))
                            ],
                          ),
                        ))))));
    return scaffold;
  }
}

class EmailTextField extends StatefulWidget {
  EmailTextField({required this.ctrl});
  final TextEditingController ctrl;
  @override
  State<StatefulWidget> createState() {
    return EmailTextFieldState();
  }
}

class EmailTextFieldState extends State<EmailTextField> {
  final emailValidateKey = GlobalKey<FormState>();
  bool emailValidate = true;
  @override
  Widget build(BuildContext context) {
    Widget suffix;
    double validateErrorH = 0.0;
    double textFiledH = 40.0.r;
    if (emailValidate == false) {
      validateErrorH = 24.r;
    }
    if (emailValidate == false && widget.ctrl.text.isNotEmpty) {
      Icon icon = const Icon(
        Icons.clear,
        color: AppColors.delete,
      );
      suffix = IconButton(
        onPressed: () {
          emailValidateKey.currentState?.reset();
          setState(() {
            widget.ctrl.clear();
          });
        },
        icon: icon,
        iconSize: 20.r,
      );
      textFiledH += validateErrorH;
    } else if (emailValidate && widget.ctrl.text.isNotEmpty) {
      suffix = Icon(
        Icons.check,
        color: AppColors.darkSkyBlue,
        size: 20.r,
      );
      textFiledH += validateErrorH;
    } else {
      suffix = SizedBox(
        width: 26.r,
        height: 24.r,
      );
      textFiledH += validateErrorH;
    }
    return Form(
        key: emailValidateKey,
        child: Column(
          children: [
            SizedBox(
                width: 282.r,
                height: 18.r,
                child: Text(
                  "Email",
                  style: TextStyle(
                    color: AppColors.greyBlue,
                    fontSize: 16.sp,
                    fontFamily: "Ubuntu",
                    fontWeight: FontWeight.w400,
                  ),
                )),
            Padding(padding: EdgeInsets.only(top: 8.r)),
            SizedBox(
                width: 282.r,
                height: textFiledH,
                child: TextFormField(
                  onChanged: (value) {
                    emailValidateKey.currentState?.validate();
                  },
                  onEditingComplete: () {
                    emailValidateKey.currentState?.validate();
                    FocusScopeNode currentFocus = FocusScope.of(context);
                    currentFocus.unfocus();
                  },
                  validator: (String? val) {
                    if (val == null || val.isEmpty) {
                      setState(() {
                        emailValidate = true;
                      });
                      return null;
                    }
                    if (EmailValidator.validate(val) == false) {
                      setState(() {
                        emailValidate = false;
                      });
                    } else {
                      setState(() {
                        emailValidate = true;
                      });
                    }
                    if (emailValidate == false) {
                      return "Incorrect email";
                    } else {
                      return null;
                    }
                  },
                  controller: widget.ctrl,
                  decoration: InputDecoration(
                    suffixIcon: suffix,
                    focusedBorder: UnderlineInputBorder(
                        borderSide:
                            BorderSide(color: AppColors.greyBlue, width: 2.r)),
                    border: UnderlineInputBorder(
                        borderSide:
                            BorderSide(color: AppColors.lightBlue, width: 1.r)),
                  ),
                  style: TextStyle(
                      color: AppColors.greyBlue,
                      fontWeight: FontWeight.w500,
                      fontFamily: "PingFangTC",
                      fontStyle: FontStyle.normal,
                      fontSize: 18.0.sp),
                ))
          ],
        ));
  }
}

class PasswordTextField extends StatefulWidget {
  PasswordTextField(
      {required Key key, required this.ctr, required this.isConfirmPassword})
      : super(key: key);
  final TextEditingController ctr;
  bool isConfirmPassword;
  @override
  State<StatefulWidget> createState() {
    return PasswordTextFieldState();
  }
}

class PasswordTextFieldState extends State<PasswordTextField> {
  bool hidePassword = true;
  final passwordKey = GlobalKey<FormState>();
  final passwordValidate = false;

  @override
  Widget build(BuildContext context) {
    double validateErrorH = 0;
    double textFieldH = 52.r;
    if (passwordValidate == false && widget.ctr.text.isNotEmpty) {
      validateErrorH = 24.r;
      textFieldH += validateErrorH;
    } else if (passwordValidate && widget.ctr.text.isNotEmpty) {
      textFieldH = validateErrorH;
    } else {
      validateErrorH = 24.r;
      textFieldH += validateErrorH;
    }
    String title;
    if (widget.isConfirmPassword) {
      title = "Confirm password";
    } else {
      title = "Password";
    }
    return Form(
      key: passwordKey,
      child: Column(
        children: [
          SizedBox(
              width: 282.r,
              height: 16.r,
              child: Text(
                title,
                style: TextStyle(
                  color: AppColors.greyBlue,
                  fontSize: 16.sp,
                  fontFamily: "Ubuntu",
                  fontWeight: FontWeight.w400,
                ),
              )),
          SizedBox(
            width: 282.r,
            height: textFieldH,
            child: TextFormField(
              scrollPadding: EdgeInsets.all(100.r),
              onEditingComplete: () {
                passwordKey.currentState?.validate();
                FocusScopeNode currentFocus = FocusScope.of(context);
                currentFocus.unfocus();
              },
              onChanged: (val) {
                passwordKey.currentState?.validate();
              },
              validator: (String? val) {
                if (val == null || widget.ctr.text.isEmpty) {
                  return null;
                }
                final reg =
                    RegExp(r'^[a-zA-Z0-9`\-=[\];,.~!@#$^&*()_+{}|:<>?]*$');
                if (reg.hasMatch(val)) {
                  return null;
                } else {
                  return "incorrectPassword" + "`-=[];,.~!@#\$^&*()_+{}|:<>?";
                }
              },
              controller: widget.ctr,
              obscureText: hidePassword,
              decoration: InputDecoration(
                suffixIcon: IconButton(
                  padding: EdgeInsets.zero,
                  icon: Icon(
                    CupertinoIcons.eye,
                    size: 20.r,
                  ),
                  onPressed: () {
                    setState(() {
                      hidePassword = !hidePassword;
                    });
                  },
                ),
                focusedBorder: UnderlineInputBorder(
                    borderSide:
                        BorderSide(color: AppColors.greyBlue, width: 2.r)),
                border: UnderlineInputBorder(
                    borderSide:
                        BorderSide(color: AppColors.lightBlue, width: 1.r)),
              ),
            ),
          )
        ],
      ),
    );
  }
}
