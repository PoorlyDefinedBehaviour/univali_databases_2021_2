import { Link } from "react-router-dom";

export const Home = () => (
  <ul className="h-screen flex justify-center items-center bg-gray-100">
    <li className="m-3">
      <Link to="/condominiums" className="inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white">Condomínios</Link>
    </li>
    <li className="m-3">
      <Link to="/employees" className="inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white">Funcionários</Link>
    </li>
  </ul>
)