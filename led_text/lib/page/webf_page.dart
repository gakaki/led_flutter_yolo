import 'package:flutter/material.dart';
import 'package:webf/webf.dart';
import 'package:webf/devtools.dart';

class WebFPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final MediaQueryData queryData = MediaQuery.of(context);
    final Size viewportSize = queryData.size;

    return Scaffold(
        body: Center(
          child: Column(
            children: [
              WebF(
                devToolsService: ChromeDevToolsService(), // Enable Chrome DevTools Services
                viewportWidth: viewportSize.width - queryData.padding.horizontal, // Adjust the viewportWidth
                viewportHeight: viewportSize.height - queryData.padding.vertical, // Adjust the viewportHeight
                bundle: WebFBundle.fromUrl('assets://main.js'), // The page entry point
              ),
            ],
          ),
        ));
  }
}
