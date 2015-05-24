function genGraph(n)
    density = 0.1;
    A = sprand(n, n, density);
    A(logical(eye(n))) = 0;
    [row,col,v] = find(A);
    csvwrite('test.csv', [row-1, col-1, v]);
end