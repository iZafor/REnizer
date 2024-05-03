"use client";

import { NavigationMenuComp as NavigationMenu } from "@/components/ui/home/navigation-menu";
import RadialGradient from "@/components/ui/magicui/radial-gradient";
import LoginForm from "@/components/ui/home/login-form";

export default function Home() {
  return (
    <div className="grid place-items-center">
      <NavigationMenu title="New Here?" />
      <RadialGradient className="w-full p-[4.3rem] md:p-[5.2rem] lg:p-[10rem] grid place-items-center">
        <h1 className="text-balance bg-gradient-to-br from-black from-30% to-black/60 bg-clip-text py-6 font-semibold leading-none tracking-tighter text-transparent dark:from-white dark:to-white/40 msm:text-4xl mmd:text-[3.5rem] text-6xl text-center">Join the Movement Towards Sustainable Energy Solutions with REnizer
        </h1>
        <LoginForm />
      </RadialGradient>
    </div>
  );
}
