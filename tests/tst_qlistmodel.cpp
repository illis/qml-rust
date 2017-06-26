/*
 * Copyright (C) 2016 Lucien XU <sfietkonstantin@free.fr>
 *
 * You may use this file under the terms of the BSD license as follows:
 *
 * "Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * The names of its contributors may not be used to endorse or promote
 *     products derived from this software without specific prior written
 *     permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
 */

#include <DOtherSideExtra/deqbaselistmodel.h>
#include <QObject>
#include <QtTest/QSignalSpy>
#include <QtTest/QTest>

class TestQListModel : public QObject
{
public:
    Q_OBJECT
private slots:
    static QVariant mapGet(const DEQBaseListModel::Data &value, int key)
    {
        auto it = value.find(key);
        if (it != value.end()) {
            return it->second;
        } else {
            return QVariant{};
        }
    }

    void testRoleNames()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        QCOMPARE(model.roleNames().size(), 2);
        QCOMPARE(model.roleNames().value(1), QByteArray{"data"});
        QCOMPARE(model.roleNames().value(2), QByteArray{"otherData"});
    }
    void testInsertAndDataAccess()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QCOMPARE(model.count(), 2);
        QCOMPARE(model.rowCount(), 2);
        QCOMPARE(model.data(model.index(0), 1), QVariant{123});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(0), 3), QVariant{});
        QCOMPARE(model.data(model.index(1), 1), QVariant{456});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 2"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{});
        QCOMPARE(model.data(model.index(-1), 1), QVariant{});
    }
    void testGet()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QCOMPARE(mapGet(model.get(0), 1), QVariant{123});
        QCOMPARE(mapGet(model.get(2), 1), QVariant{});
        QCOMPARE(mapGet(model.get(-1), 1), QVariant{});
    }
    void testSet()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QSignalSpy spy{&model, SIGNAL(dataChanged(QModelIndex, QModelIndex, QVector<int>))};
        QVERIFY(model.set(1, {{{1, QVariant{789}}, {3, QVariant{123.456}}}}));

        QCOMPARE(spy.size(), 1);
        QCOMPARE(model.data(model.index(1), 1), QVariant{789});
        QCOMPARE(model.data(model.index(1), 2), QVariant{});
        QCOMPARE(model.data(model.index(1), 3), QVariant{});
    }
    void testSetInvalid()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        QVERIFY(!model.set(0, {}));
        QVERIFY(!model.set(-1, {}));
    }
    void testInsert1()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QVERIFY(model.insert(0, {{{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                                 {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}}}));

        QCOMPARE(model.rowCount(), 4);
        QCOMPARE(model.data(model.index(0), 1), QVariant{789});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 3"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{123});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 4"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{123});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(3), 1), QVariant{456});
        QCOMPARE(model.data(model.index(3), 2), QVariant{QLatin1String{"Test 2"}});
    }
    void testInsert2()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QVERIFY(model.insert(2, {{{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                                 {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}}}));

        QCOMPARE(model.rowCount(), 4);
        QCOMPARE(model.data(model.index(0), 1), QVariant{123});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{456});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 2"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{789});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 3"}});
        QCOMPARE(model.data(model.index(3), 1), QVariant{123});
        QCOMPARE(model.data(model.index(3), 2), QVariant{QLatin1String{"Test 4"}});
    }
    void testInsert3()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}});

        QVERIFY(model.insert(1, {{{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                                 {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}}}));

        QCOMPARE(model.rowCount(), 4);
        QCOMPARE(model.data(model.index(0), 1), QVariant{123});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{789});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 3"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{123});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 4"}});
        QCOMPARE(model.data(model.index(3), 1), QVariant{456});
        QCOMPARE(model.data(model.index(3), 2), QVariant{QLatin1String{"Test 2"}});
    }
    void testInsert4()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        QSignalSpy spy1{&model, SIGNAL(rowsAboutToBeInserted(QModelIndex, int, int))};
        QSignalSpy spy2{&model, SIGNAL(rowsInserted(QModelIndex, int, int))};

        QVERIFY(model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                                 {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}}}));

        QCOMPARE(spy1.size(), 1);
        QCOMPARE(spy2.size(), 1);
    }

    void testInsertInvalid1()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        QSignalSpy spy1{&model, SIGNAL(rowsAboutToBeInserted(QModelIndex, int, int))};
        QSignalSpy spy2{&model, SIGNAL(rowsInserted(QModelIndex, int, int))};

        QVERIFY(!model.insert(0, {}));

        QCOMPARE(spy1.size(), 0);
        QCOMPARE(spy2.size(), 0);
    }
    void testInsertInvalid2()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        QSignalSpy spy1{&model, SIGNAL(rowsAboutToBeInserted(QModelIndex, int, int))};
        QSignalSpy spy2{&model, SIGNAL(rowsInserted(QModelIndex, int, int))};

        QVERIFY(!model.insert(-1, {}));
        QVERIFY(!model.insert(1, {}));

        QCOMPARE(spy1.size(), 0);
        QCOMPARE(spy2.size(), 0);
    }
    void testRemove1()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}},
                         {{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                         {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 5"}}}}});

        QVERIFY(model.remove(0, 2));

        QCOMPARE(model.rowCount(), 3);
        QCOMPARE(model.data(model.index(0), 1), QVariant{789});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 3"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{123});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 4"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{456});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 5"}});
    }
    void testRemove2()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}},
                         {{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                         {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 5"}}}}});

        QVERIFY(model.remove(1, 2));

        QCOMPARE(model.rowCount(), 3);
        QCOMPARE(model.data(model.index(0), 1), QVariant{123});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{123});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 4"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{456});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 5"}});
    }
    void testRemove3()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}},
                         {{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                         {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 5"}}}}});

        QVERIFY(model.remove(3, 2));

        QCOMPARE(model.rowCount(), 3);
        QCOMPARE(model.data(model.index(0), 1), QVariant{123});
        QCOMPARE(model.data(model.index(0), 2), QVariant{QLatin1String{"Test 1"}});
        QCOMPARE(model.data(model.index(1), 1), QVariant{456});
        QCOMPARE(model.data(model.index(1), 2), QVariant{QLatin1String{"Test 2"}});
        QCOMPARE(model.data(model.index(2), 1), QVariant{789});
        QCOMPARE(model.data(model.index(2), 2), QVariant{QLatin1String{"Test 3"}});
    }
    void testRemove4()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}},
                         {{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                         {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 5"}}}}});

        QSignalSpy spy1{&model, SIGNAL(rowsAboutToBeRemoved(QModelIndex, int, int))};
        QSignalSpy spy2{&model, SIGNAL(rowsRemoved(QModelIndex, int, int))};

        QVERIFY(model.remove(1, 2));

        QCOMPARE(spy1.size(), 1);
        QCOMPARE(spy2.size(), 1);
    }
    void testRemoveInvalid()
    {
        DEQBaseListModel model{{{1, "data"}, {2, "otherData"}}};
        model.insert(0, {{{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 1"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 2"}}}},
                         {{1, QVariant{789}}, {2, QVariant{QLatin1String{"Test 3"}}}},
                         {{1, QVariant{123}}, {2, QVariant{QLatin1String{"Test 4"}}}},
                         {{1, QVariant{456}}, {2, QVariant{QLatin1String{"Test 5"}}}}});

        QSignalSpy spy1{&model, SIGNAL(rowsAboutToBeRemoved(QModelIndex, int, int))};
        QSignalSpy spy2{&model, SIGNAL(rowsRemoved(QModelIndex, int, int))};

        QVERIFY(!model.remove(-1, 1));
        QVERIFY(!model.remove(0, -1));
        QVERIFY(!model.remove(0, 0));
        QVERIFY(!model.remove(5, 1));
        QVERIFY(!model.remove(4, 2));

        QCOMPARE(spy1.size(), 0);
        QCOMPARE(spy2.size(), 0);
    }

    void test()
    {
        std::vector<std::string> values{"Hello", "world"};
        std::string &first = values[0];
        values.push_back("foo");
    }
};

QTEST_MAIN(TestQListModel)

#include "tst_qlistmodel.moc"
