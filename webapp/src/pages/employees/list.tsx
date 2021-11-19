import { useState } from "react"
import { useQuery } from "react-query"
import { useHistory } from "react-router-dom"
import { deleteById, Employee, getAll } from "../../services/employees"


type Props = {
  data: Employee
  onEdit: () => void
  onDelete: () => void
}

const Card = ({ data, onEdit, onDelete}: Props) => (
  <div className="max-w-md py-4 px-8 bg-white shadow-lg rounded-lg my-20">
    <div>
      <h2 className="text-gray-800 text-3xl font-semibold">{data.name}</h2>
      <h2 className="text-gray-800 text-2xl font-semibold">{data.role} - {data.shift}</h2>
      <p className="mt-2 text-gray-600">Rua {data.address.street}, {data.address.city.name}, {data.address.city.state.name}</p>
    </div>
    <div className="flex justify-end mt-4">
      <p>CPF {data.cpf}</p>
    </div>
    <div className="flex justify-end mt-4">
      <p>Salário {data.wage_in_cents / 100}</p>
    </div>
    <div className="flex justify-between">
      <button 
        className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
        onClick={onEdit}
      >
        Editar
      </button>
      <button 
        className="flex justify-end mt-4 bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white py-2 px-4 border border-red-500 hover:border-transparent rounded"
        onClick={onDelete}
      >
        Excluir
      </button>
    </div>
  </div>
)
  
export const Employees = () => {
  const [editing, setEditing] = useState<null | Employee>(null)
  const { isLoading, data: employees } = useQuery("get_all_employees", getAll)
  const history = useHistory()

  if(isLoading) {
    return <p>...</p>
  }

  if(editing !== null) {
    // return <Editemployee employee={editing}/>
  }

  const handleDelete = (employeeId: number) => {
    deleteById(employeeId)
      .then(() => history.push("/"))
  }

  return (
   <div>
     <button 
      className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
      onClick={() => history.push("/employees/create")}
      >
        Criar
      </button>
     { employees?.map(employee =>
         <Card 
            key={employee.id}
            data={employee} 
            onEdit={() => setEditing(employee)}
            onDelete={() => handleDelete(employee.id)}
          />) 
     }
   </div>
   )
}