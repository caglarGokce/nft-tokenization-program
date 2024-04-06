'use client';

import React, { useState } from 'react';
import Button from '../Button';
import { useEffectOnce } from 'usehooks-ts';

export default function ScrollToTopButton() {
  const [vis, setVis] = useState(false);

  useEffectOnce(() => {
    const listener = () => {
      const isOver = window.scrollY > window.innerHeight;
      if (isOver) {
        setVis(true);
      } else if (!isOver) {
        setVis(false);
      }
    };
    document.addEventListener('scrollend', listener);
    return () => document.removeEventListener('scrollend', listener);
  });

  const handleClick = () => {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  return (
    <Button
      variant="contained"
      onClick={handleClick}
      className={
        'transition-[right] fixed bottom-6' + (!vis ? ' -right-12' : ' right-6')
      }
      size="sm"
    />
  );
}
