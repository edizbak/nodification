import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'loutre-base';

   login(){
    alert("log me in");
  }

  signup(){
    alert("Sign me up");
  }
  
}
