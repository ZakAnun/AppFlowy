import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flutter/material.dart';
import 'package:flowy_infra/size.dart';

import 'base_styled_button.dart';

enum TextButtonMode {
  normal,
  big,
  small;

  Size get size {
    switch (this) {
      case TextButtonMode.normal:
        return const Size(80, 32);
      case TextButtonMode.big:
        return const Size(100, 40);
      case TextButtonMode.small:
        return const Size(100, 30);
    }
  }

  BorderRadius get borderRadius {
    switch (this) {
      case TextButtonMode.normal:
        return Corners.s8Border;
      case TextButtonMode.big:
        return Corners.s12Border;
      case TextButtonMode.small:
        return Corners.s6Border;
    }
  }
}

class SecondaryTextButton extends StatelessWidget {
  const SecondaryTextButton(
    this.label, {
    super.key,
    this.onPressed,
    this.mode = TextButtonMode.normal,
  });

  final String label;
  final VoidCallback? onPressed;
  final TextButtonMode mode;

  @override
  Widget build(BuildContext context) {
    return SecondaryButton(
      mode: mode,
      onPressed: onPressed,
      child: FlowyText.regular(
        label,
        color: Theme.of(context).colorScheme.primary,
      ),
    );
  }
}

class SecondaryButton extends StatelessWidget {
  const SecondaryButton({
    super.key,
    required this.child,
    this.onPressed,
    this.mode = TextButtonMode.normal,
  });

  final Widget child;
  final VoidCallback? onPressed;
  final TextButtonMode mode;

  @override
  Widget build(BuildContext context) {
    final size = mode.size;
    return BaseStyledButton(
      minWidth: size.width,
      minHeight: size.height,
      contentPadding: EdgeInsets.zero,
      bgColor: Colors.transparent,
      outlineColor: Theme.of(context).colorScheme.onBackground,
      borderRadius: mode.borderRadius,
      onPressed: onPressed,
      child: child,
    );
  }
}
