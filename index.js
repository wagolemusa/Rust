// const item = document.getElementById("Mydiv");
// item.innerHTML = "hello ... refuge Wise"

// const byClass = document.getElementsByClassName("MyClass")

// byClass[0].innerHTML = "Welecome my Niggah"
// byClass[1].innerHTML = "Welecome my Niggah"

// const myTag = document.getElementsByTagName("h2")
// myTag.innerHTML = "This is Where Hip Starts"


// function names() {
//     const cars = ["Suburu", "Volvo", "BMW"]
//     const item = document.getElementById("Mydiv");

//     for(let i = 0; i < cars.length; i++){
//         const me = cars[i]
//         const node = document.createElement("li");
//         const textnode = document.createTextNode(me);
//         node.appendChild(textnode);
//         item.appendChild(node);

//   }

    // cars.forEach((data) =>{
    //     const me = data
    //     const node = document.createElement("li");
    //     const textnode = document.createTextNode(me);
    //     node.appendChild(textnode);
    //     item.appendChild(node);
    // })


  
  
// }

// names()


const source = document.getElementById("inputdata");

const outsource = document.getElementById("getText");


const mydata = function (e) {
    outsource.innerHTML = e.target.value;
    
}
source.addEventListener("input", mydata);
