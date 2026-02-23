'use client';

import { useAuth } from '@/contexts/AuthContext';
import { Box, Button, Container, Flex, Heading, Text } from '@radix-ui/themes';

export default function Header() {
  const { user, logout } = useAuth();

  return (
    <Box
      px="3"
      style={{
        borderBottom: '1px solid var(--gray-5)',
        backgroundColor: 'var(--color-background)',
      }}
    >
      <Container size="3">
        <Flex justify="between" align="center">
          <Heading size="4">ToDo App</Heading>
          <Flex align="center" gap="3">
            <Text size="2" color="gray">
              {user?.email}
            </Text>
            <Button variant="soft" size="1" onClick={logout}>
              ログアウト
            </Button>
          </Flex>
        </Flex>
      </Container>
    </Box>
  );
}
