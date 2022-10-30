import {React, useEffect, useState} from "react";
import { Link } from "react-router-dom";

function Home() {

   let [data, setData] = useState([])

   let graphul_data = async () => {
    const response = await fetch(`http://127.0.0.1:8000/api/articles`);
    const data = await response.json();
    setData(data)
   }

   useEffect(() => {
     graphul_data()
   }, [])

  return (
    <div className="h-10">
        <section className="bg-white dark:bg-gray-900">
            <div className="container px-6 py-10 mx-auto">
                <h1 className="text-3xl font-semibold text-gray-800 capitalize lg:text-4xl dark:text-white">Graphul Blog</h1>
                <div className="grid grid-cols-1 gap-8 mt-8 md:mt-16 md:grid-cols-2">
                    {data.map((item) =>
                         <div className="lg:flex" key={item.id}>
                            <img className="object-cover w-full h-56 rounded-lg lg:w-64" src={item.img} alt="" />

                            <div className="flex flex-col justify-between py-6 lg:mx-6">
                                <Link to={`/${item.id}`} className="text-xl font-semibold text-gray-800 hover:underline dark:text-white ">
                                    {item.title}
                                </Link>

                                <span className="text-sm text-gray-500 dark:text-gray-300">Author: {item.user_name}</span>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </section>
    </div>
  );
}

export default Home;
