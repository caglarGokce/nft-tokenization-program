'use client';

import React, { HTMLProps, useEffect, useMemo, useState } from 'react';
import Surface from '../Surface';
import Sidebar from './Sidebar';
import Header from './Header';
import { usePathname, useRouter } from 'next/navigation';
import Stack from '../Stack';
import Logo from '../Logo';
import { Provider as ReduxProvider } from 'react-redux';
import Typography from '../Typography';
import ScrollToTopButton from '../ScrollToTopButton';
import { useTheme } from '@/hooks/theme';
import { useBreakpointMatch } from '@/hooks/useBreakpointMatch';
import { navRoutes } from '@/routes';
import store from '@/store';

export default function Layout(props: Readonly<HTMLProps<HTMLDivElement>>) {
  return (
    <ReduxProvider store={store}>
      <Container {...props} />
    </ReduxProvider>
  );
}

/** Loading messages */
const messages = {
  /** Initial state */
  start: 'Hold on',
  /** Session token loaded and verified */
  sessionLoad: 'Logged in!',
  /** Invalid or no token present, final state */
  finish: 'Finishing up',
};

function Container({ children }: Readonly<HTMLProps<HTMLDivElement>>) {
  const {
    colors,
    dimensions: { sidebar, headerMobile },
  } = useTheme();
  const router = useRouter();
  const pathname = usePathname();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [loading, setLoading] = useState<boolean>(false);
  const [message, setMessage] = useState(messages.start);
  const isMd = useBreakpointMatch('md');

  const Component = useMemo(() => {
    let Component = (
      <>
        <Sidebar
          routes={navRoutes}
          open={sidebarOpen}
          onClose={() => setSidebarOpen(false)}
        />
        <Stack
          spacing={2}
          className={`md:p-6 min-h-screen`}
          style={{marginLeft: sidebar.width,marginTop: headerMobile.height}}
        >
          <Header onOpenMenu={() => setSidebarOpen(true)} />
          <div className="my-5 mx-4 md:m-0 md:mt-11">{children}</div>
        </Stack>
      </>
    );
    if (loading || loading === undefined)
      Component = (
        <Stack className="dark bg-surfaceDark w-screen h-screen items-center justify-center space-y-4">
          <div className="animate-pulse">
            <Logo width={64} height={64} />
          </div>
          <Typography text={message} color={colors.typographyDark.DEFAULT} />
        </Stack>
      );
    return Component;
  }, [
    pathname,
    router,
    loading,
    children,
    message,
    sidebar.width,
    headerMobile.height,
    sidebarOpen,
    isMd,
    colors.typographyDark.DEFAULT,
  ]);

  return (
    <MainWrapper>
      <Surface>{Component}</Surface>
      <ScrollToTopButton />
    </MainWrapper>
  );
}

const MainWrapper = (props: HTMLProps<HTMLElement>) => {
  const { isDark } = useTheme();
  const [classNames, setClassNames] = useState('');

  useEffect(() => {
    if (isDark) setClassNames('dark');
    else setClassNames('');
  }, [isDark]);

  return (
    <main className={classNames}>
      {props.children}
    </main>
  );
};
