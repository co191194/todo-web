import { Container, Heading, Text, Box } from "@radix-ui/themes";

export default function Home() {
  return (
    <Container size="2" py="9">
      <Box mb="6">
        <Heading size="8" mb="2">
          ToDo App
        </Heading>
        <Text color="gray" size="4">
          シンプルなToDo管理アプリケーション
        </Text>
      </Box>

      <Box p="4" style={{ backgroundColor: "var(--gray-2)", borderRadius: "var(--radius-3)" }}>
        <Text>✅ フロントエンドが正常に起動しています</Text>
      </Box>
    </Container>
  );
}
