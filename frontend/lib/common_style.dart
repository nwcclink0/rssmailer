import 'package:flutter/material.dart';
import 'package:flutter_screenutil/flutter_screenutil.dart';
import 'package:rssmailer/app_style.dart';

class ConfirmBtn extends StatelessWidget {
  const ConfirmBtn(
      {required Key key,
      required this.title,
      required this.onPress,
      required this.disable})
      : super(key: key);
  final String title;
  final Function onPress;
  final bool disable;

  @override
  Widget build(BuildContext context) {
    Color buttonColor = AppColors.lightBlue;
    Color enableColor = AppColors.darkSkyBlue;
    if (disable == false) {
      buttonColor = enableColor;
    }
    return SizedBox(
        width: 228.r,
        height: 40.r,
        child: GestureDetector(
          onTap: () {
            onPress();
          },
          child: Container(
              decoration: BoxDecoration(
                  borderRadius: BorderRadius.all(Radius.circular(100.r)),
                  color: buttonColor),
              child: Center(
                child: Text(
                  title,
                  style: TextStyle(
                      color: const Color(0xffffffff),
                      fontWeight: FontWeight.w400,
                      fontFamily: "Ubuntu",
                      fontStyle: FontStyle.normal,
                      fontSize: 16.0.sp),
                ),
              )),
        ));
  }
}
