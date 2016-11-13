import 'package:angular2/core.dart';
import 'package:angular2/router.dart';
import 'package:portfolio/navbar_component.dart';
import 'package:portfolio/home/home_component.dart';
import 'package:portfolio/admin/admin_component.dart';


@Component(
    selector: 'my-app',
    templateUrl: 'app_component.html',
    directives: const [ROUTER_DIRECTIVES, NavbarComponent])
@RouteConfig(const [const Route(path: '/', component: HomeComponent, name: "Home"),
                    const Route(path: '/admin', component: AdminComponent, name: "Admin")])
class AppComponent {}
