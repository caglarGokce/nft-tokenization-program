'use client';
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';

/** Renders the home page */
const Home = () => {
  const router = useRouter();

  useEffect(() => {
    router.push('/dashboard');
  }, [router]);
  return <></>;
};

export default Home;
