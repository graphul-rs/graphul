import {React, useEffect, useState} from "react";
import { useParams } from "react-router-dom";

export default function Detail() {
    let [data, setData] = useState(null)
    let { id } = useParams();

   let graphul_data = async () => {
    const response = await fetch(`http://127.0.0.1:8000/api/article/${id}`);
    const data = await response.json();
    setData(data)
   }

   useEffect(() => {
    graphul_data()
   }, [])

    return (
        <div className="h-screen">
            <section className="bg-white h-screen dark:bg-gray-900">
                { !data && <h1 className="text-4xl"> Loading.. </h1>}
                { data &&
                 <div className="container px-6 py-10 mx-auto">
                    <h1 className="text-3xl font-semibold text-gray-800 capitalize lg:text-4xl dark:text-white">{data.title}</h1>

                    <div className="grid gap-8 mt-8 md:mt-16">
                        <div className="lg:flex">
                            <img className="object-cover w-full h-56 justify-center rounded-lg lg:w-64" src={data.img} alt="" />

                            <div className="flex flex-col justify-between py-6 lg:mx-6">
                                <p className="dark:text-white">{data.body}</p>
                                <span className="text-sm text-gray-500 dark:text-gray-300">Author: {data.user_name}</span>
                            </div>
                        </div>
                    </div>
                </div>}
            </section>
        </div>
      );
}
