import Navbar from "@/components/ui/dashboard/navbar"

export default function DashboardLayout({ children }: { children?: React.ReactNode }) {
    return (
        <>
            <Navbar />
            {children}
        </>
    )
}