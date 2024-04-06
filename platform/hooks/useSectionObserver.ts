import { sidebarActions } from '@/store/actions';
import { useEffect, RefObject } from 'react';
import { useDispatch } from 'react-redux';

type SectionObserverProps = {
  ref: RefObject<HTMLElement>;
  sectionId: string;
  path: string;
  mainPath: string;
};

const useSectionObserver = ({
  ref,
  sectionId,
  path,
  mainPath,
}: SectionObserverProps) => {
  const dispatch = useDispatch();

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          dispatch(sidebarActions.changeSubPath({ path, mainPath }));
          window.history.replaceState(null, '', `#${sectionId}`);
        }
      },
      { root: null, threshold: 0.1 },
    );

    if (ref.current) {
      observer.observe(ref.current);
    }

    return () => {
      if (ref.current) {
        observer.unobserve(ref.current);
      }
    };
  }, [ref, sectionId, path, mainPath, dispatch]);
};

export default useSectionObserver;
