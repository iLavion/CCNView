// app/ui/shared/Navigation.tsx

import { Link } from "react-router";

export default function Navigation() {
  return (
    <header className="bg-white shadow-sm">
        <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
        <div className="font-bold text-2xl text-blue-600">Invalsia</div>
        <div className="flex space-x-4">
            <a href="#" className="text-gray-700 hover:text-blue-600">About</a>
            <a href="#" className="text-gray-700 hover:text-blue-600">Features</a>
            <a href="#" className="text-gray-700 hover:text-blue-600">Contact</a>
        </div>
        </nav>
    </header>
  )
}